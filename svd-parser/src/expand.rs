//! Provides [expand] method to convert arrays, clusters and derived items in regular instances

use anyhow::{anyhow, Result};
use std::borrow::Cow;
use std::collections::HashMap;
use std::mem::take;
use svd_rs::{
    array::names, cluster, field, peripheral, register, BitRange, Cluster, ClusterInfo, DeriveFrom,
    Device, EnumeratedValues, Field, Peripheral, Register, RegisterCluster, RegisterProperties,
};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct RegisterPath {
    pub peripheral: String,
    pub path: Vec<String>,
}

impl RegisterPath {
    pub fn new(p: &str) -> Self {
        Self {
            peripheral: p.into(),
            path: Vec::new(),
        }
    }
    pub fn new_child(&self, name: impl Into<String>) -> Self {
        let mut child = self.clone();
        child.path.push(name.into());
        child
    }
    pub fn split_str<'a>(s: &'a str) -> (Option<Self>, Cow<'a, str>) {
        Self::split_vec(s.split('.').collect())
    }
    pub fn split_vec<'a>(mut v: Vec<&'a str>) -> (Option<Self>, Cow<'a, str>) {
        let name = v.pop().unwrap();
        let mut iter = v.into_iter();
        let path = if let Some(p) = iter.next() {
            let mut rpath = Self::new(p);
            rpath.path = iter.map(Into::into).collect();
            Some(rpath)
        } else {
            None
        };
        (path, name.into())
    }
    pub fn parent(&self) -> Self {
        let mut parent = self.clone();
        parent.path.pop();
        parent
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct FieldPath {
    pub register: RegisterPath,
    pub name: String,
}

impl FieldPath {
    pub fn new(r: &RegisterPath, name: &str) -> Self {
        Self {
            register: r.clone(),
            name: name.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct EnumPath<'a> {
    pub field: FieldPath,
    pub name: &'a str,
}

impl<'a> EnumPath<'a> {
    pub fn new(f: &FieldPath, name: &'a str) -> Self {
        Self {
            field: f.clone(),
            name,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Index<'a> {
    pub peripherals: HashMap<Cow<'a, str>, &'a Peripheral>,
    pub clusters: HashMap<RegisterPath, &'a Cluster>,
    pub registers: HashMap<RegisterPath, &'a Register>,
    pub fields: HashMap<FieldPath, &'a Field>,
    pub evs: HashMap<EnumPath<'a>, &'a EnumeratedValues>,
}

impl<'a> Index<'a> {
    fn add_peripheral(&mut self, p: &'a Peripheral) {
        if let Peripheral::Array(info, dim) = p {
            for name in names(info, dim) {
                let path = RegisterPath::new(&name);
                for r in p.registers() {
                    self.add_register(&path, r);
                }
                for c in p.clusters() {
                    self.add_cluster(&path, c);
                }
                self.peripherals.insert(name.into(), p);
            }
        }
        let path = RegisterPath::new(&p.name);
        for r in p.registers() {
            self.add_register(&path, r);
        }
        for c in p.clusters() {
            self.add_cluster(&path, c);
        }
        self.peripherals.insert(p.name.as_str().into(), p);
    }

    fn add_cluster(&mut self, path: &RegisterPath, c: &'a Cluster) {
        if let Cluster::Array(info, dim) = c {
            for name in names(info, dim) {
                let cpath = RegisterPath::new_child(path, name);
                for r in c.registers() {
                    self.add_register(&cpath, r);
                }
                for c in c.clusters() {
                    self.add_cluster(&cpath, c);
                }
                self.clusters.insert(cpath, c);
            }
        }
        let cpath = RegisterPath::new_child(path, &c.name);
        for r in c.registers() {
            self.add_register(&cpath, r);
        }
        for c in c.clusters() {
            self.add_cluster(&cpath, c);
        }
        self.clusters.insert(cpath, c);
    }
    fn add_register(&mut self, path: &RegisterPath, r: &'a Register) {
        if let Register::Array(info, dim) = r {
            for name in names(info, dim) {
                let rpath = RegisterPath::new_child(path, name);
                for f in r.fields() {
                    self.add_field(&rpath, f);
                }
                self.registers.insert(rpath, r);
            }
        }
        let rpath = RegisterPath::new_child(path, &r.name);
        for f in r.fields() {
            self.add_field(&rpath, f);
        }
        self.registers.insert(rpath, r);
    }
    fn add_field(&mut self, path: &RegisterPath, f: &'a Field) {
        if let Field::Array(info, dim) = f {
            for name in names(info, dim) {
                let fpath = FieldPath::new(path, &name);
                for evs in &f.enumerated_values {
                    if let Some(name) = evs.name.as_ref() {
                        self.evs.insert(EnumPath::new(&fpath, name), evs);
                    }
                }
                self.fields.insert(fpath, f);
            }
        }
        let fpath = FieldPath::new(path, &f.name);
        for evs in &f.enumerated_values {
            if let Some(name) = evs.name.as_ref() {
                self.evs.insert(EnumPath::new(&fpath, name), evs);
            }
        }
        self.fields.insert(fpath, f);
    }

    pub fn create(device: &'a Device) -> Self {
        let mut index = Self::default();
        for p in &device.peripherals {
            index.add_peripheral(p);
        }
        index
    }
}

fn expand_register_cluster(
    regs: &mut Vec<RegisterCluster>,
    rc: RegisterCluster,
    path: &RegisterPath,
    index: &Index,
) -> Result<()> {
    match rc {
        RegisterCluster::Cluster(c) => expand_cluster_array(regs, c, path, index)?,
        RegisterCluster::Register(r) => expand_register_array(regs, r, path, index)?,
    }
    Ok(())
}

fn expand_cluster_array(
    regs: &mut Vec<RegisterCluster>,
    mut c: Cluster,
    path: &RegisterPath,
    index: &Index,
) -> Result<()> {
    let mut cpath = None;
    if let Some(dpath) = c.derived_from.as_ref() {
        let dpath = dpath.to_string();
        cpath = derive_cluster(&mut c, &dpath, path, index)?;
        c.derived_from = None;
    }
    let cpath = cpath.unwrap_or_else(|| RegisterPath::new_child(path, &c.name));

    for rc in take(&mut c.children) {
        expand_register_cluster(&mut c.children, rc, &cpath, index)?;
    }

    match c {
        Cluster::Single(c) => expand_cluster(regs, c),
        Cluster::Array(info, dim) => {
            for c in names(&info, &dim)
                .zip(cluster::address_offsets(&info, &dim))
                .map(|(name, address_offset)| {
                    let mut info = info.clone();
                    info.name = name;
                    info.address_offset = address_offset;
                    info
                })
            {
                expand_cluster(regs, c);
            }
        }
    }
    Ok(())
}

fn derive_cluster(
    c: &mut Cluster,
    dpath: &str,
    path: &RegisterPath,
    index: &Index,
) -> Result<Option<RegisterPath>> {
    let (dparent, dname) = RegisterPath::split_str(dpath);
    let rdpath;
    let cluster_path;
    let d = (if let Some(dparent) = dparent {
        cluster_path = RegisterPath::new_child(&dparent, dname);
        rdpath = dparent;
        index.clusters.get(&cluster_path)
    } else {
        cluster_path = RegisterPath::new_child(path, dname);
        rdpath = path.clone();
        index.clusters.get(&cluster_path)
    })
    .ok_or_else(|| anyhow!("cluster {} not found", dpath))?;

    let mut cpath = None;
    if c.children.is_empty() {
        cpath = Some(cluster_path);
    }
    *c = c.derive_from(d);
    if let Some(dpath) = d.derived_from.as_ref() {
        cpath = derive_cluster(c, dpath, &rdpath, index)?;
    }
    Ok(cpath)
}

fn derive_register(
    r: &mut Register,
    dpath: &str,
    path: &RegisterPath,
    index: &Index,
) -> Result<Option<RegisterPath>> {
    let (dparent, dname) = RegisterPath::split_str(dpath);
    let rdpath;
    let reg_path;
    let d = (if let Some(dparent) = dparent {
        reg_path = RegisterPath::new_child(&dparent, dname);
        rdpath = dparent;
        index.registers.get(&reg_path)
    } else {
        reg_path = RegisterPath::new_child(path, dname);
        rdpath = path.clone();
        index.registers.get(&reg_path)
    })
    .ok_or_else(|| anyhow!("register {} not found", dpath))?;

    let mut rpath = None;
    if r.fields.is_none() {
        rpath = Some(reg_path);
    }
    *r = r.derive_from(d);
    if let Some(dpath) = d.derived_from.as_ref() {
        rpath = derive_register(r, dpath, &rdpath, index)?;
    }
    Ok(rpath)
}

fn derive_field(
    f: &mut Field,
    dpath: &str,
    rpath: &RegisterPath,
    index: &Index,
) -> Result<Option<FieldPath>> {
    let (dparent, dname) = RegisterPath::split_str(dpath);
    let rdpath;
    let field_path;
    let d = (if let Some(dparent) = dparent {
        field_path = FieldPath::new(&dparent, &dname);
        rdpath = dparent;
        index.fields.get(&field_path)
    } else {
        field_path = FieldPath::new(rpath, &dname);
        rdpath = rpath.clone();
        index.fields.get(&field_path)
    })
    .ok_or_else(|| anyhow!("field {} not found", dpath))?;

    let mut fpath = None;
    if f.enumerated_values.is_empty() {
        fpath = Some(field_path);
    }
    *f = f.derive_from(d);
    if let Some(dpath) = d.derived_from.as_ref() {
        fpath = derive_field(f, dpath, &rdpath, index)?;
    }
    Ok(fpath)
}

fn expand_cluster(regs: &mut Vec<RegisterCluster>, c: ClusterInfo) {
    for rc in c.children {
        match rc {
            RegisterCluster::Cluster(_) => unreachable!(),
            RegisterCluster::Register(mut r) => {
                r.name = format!("{}_{}", c.name, r.name);
                r.address_offset += c.address_offset;
                regs.push(RegisterCluster::Register(r));
            }
        }
    }
}

fn expand_register_array(
    regs: &mut Vec<RegisterCluster>,
    mut r: Register,
    path: &RegisterPath,
    index: &Index,
) -> Result<()> {
    let mut rpath = None;
    if let Some(dpath) = r.derived_from.as_ref() {
        let dpath = dpath.to_string();
        rpath = derive_register(&mut r, &dpath, path, index)?;
        r.derived_from = None;
    }
    let rpath = rpath.unwrap_or_else(|| RegisterPath::new_child(path, &r.name));

    if let Some(field) = r.fields.as_mut() {
        for f in take(field) {
            expand_field(field, f, &rpath, index)?;
        }
    }

    match r {
        Register::Single(_) => {
            regs.push(RegisterCluster::Register(r));
        }
        Register::Array(info, dim) => {
            for rx in names(&info, &dim)
                .zip(register::address_offsets(&info, &dim))
                .map(|(name, address_offset)| {
                    let mut info = info.clone();
                    info.name = name;
                    info.address_offset = address_offset;
                    Register::Single(info)
                })
            {
                regs.push(RegisterCluster::Register(rx));
            }
        }
    }
    Ok(())
}

fn expand_field(
    fields: &mut Vec<Field>,
    mut f: Field,
    rpath: &RegisterPath,
    index: &Index,
) -> Result<()> {
    let mut fpath = None;
    if let Some(dpath) = f.derived_from.as_ref() {
        let dpath = dpath.to_string();
        fpath = derive_field(&mut f, &dpath, rpath, index)?;
        f.derived_from = None;
    }
    let fpath = fpath.unwrap_or_else(|| FieldPath::new(rpath, &f.name));

    for ev in &mut f.enumerated_values {
        if let Some(dpath) = ev.derived_from.as_ref() {
            let dpath = dpath.to_string();
            derive_enumerated_values(ev, &dpath, &fpath, index)?;
            ev.derived_from = None;
        }
    }

    match f {
        Field::Single(_) => {
            fields.push(f);
        }
        Field::Array(info, dim) => {
            for fx in
                names(&info, &dim)
                    .zip(field::bit_offsets(&info, &dim))
                    .map(|(name, bit_offset)| {
                        let mut info = info.clone();
                        info.name = name;
                        info.bit_range = BitRange::from_offset_width(bit_offset, info.bit_width());
                        Field::Single(info)
                    })
            {
                fields.push(fx);
            }
        }
    }

    Ok(())
}

fn derive_enumerated_values(
    ev: &mut EnumeratedValues,
    dpath: &str,
    fpath: &FieldPath,
    index: &Index,
) -> Result<()> {
    let mut v: Vec<&str> = dpath.split('.').collect();
    let dname = v.pop().unwrap();
    let d = if v.is_empty() {
        // Only EVNAME: Must be in one of fields in same register
        let rdpath = &fpath.register;
        if let Some(r) = index.registers.get(rdpath) {
            let mut found = None;
            for f in r.fields() {
                let fdpath = FieldPath::new(rdpath, &f.name);
                if let Some(d) = index.evs.get(&EnumPath::new(&fdpath, dname)) {
                    found = Some((d, fdpath));
                    break;
                }
            }
            found
        } else {
            None
        }
    } else {
        let fdname = v.pop().unwrap();
        let fdpath = if v.is_empty() {
            // FIELD.EVNAME
            FieldPath::new(&fpath.register, &fdname)
        } else {
            let (rdpath, rdname) = RegisterPath::split_vec(v);
            let rdpath = if let Some(mut rdpath) = rdpath {
                // FULL.PATH.EVNAME:
                rdpath.path.push(rdname.into());
                rdpath
            } else {
                // REG.FIELD.EVNAME
                let mut rdpath = fpath.register.parent();
                rdpath.path.push(rdname.into());
                rdpath
            };
            FieldPath::new(&rdpath, &fdname)
        };
        index
            .evs
            .get(&EnumPath::new(&fdpath, dname))
            .map(|d| (d, fdpath))
    };

    if let Some((d, fdpath)) = d {
        *ev = ev.derive_from(d);
        if let Some(dpath) = d.derived_from.as_ref() {
            derive_enumerated_values(ev, dpath, &fdpath, index)?;
        }
    } else {
        return Err(anyhow!(
            "enumeratedValues {} not found, parent field: {:?}",
            dpath,
            fpath,
        ));
    }
    Ok(())
}

fn derive_peripheral(
    p: &mut Peripheral,
    dpath: &str,
    index: &Index,
) -> Result<Option<RegisterPath>> {
    let mut path = None;
    let d = index
        .peripherals
        .get(dpath)
        .ok_or_else(|| anyhow!("peripheral {} not found", dpath))?;
    if p.registers.is_none() {
        path = Some(RegisterPath::new(dpath));
    }
    *p = p.derive_from(d);
    if let Some(dpath) = d.derived_from.as_ref() {
        path = derive_peripheral(p, dpath, index)?;
        p.derived_from = None;
    }
    Ok(path)
}

/// Creates clone of device with expanded arrays of peripherals, clusters, registers and fields.
/// Also resolves all `derivedFrom` reference pathes
pub fn expand(indevice: &Device) -> Result<Device> {
    let mut device = indevice.clone();

    let index = Index::create(indevice);

    let peripherals = take(&mut device.peripherals);
    for mut p in peripherals {
        let mut path = None;
        if let Some(dpath) = p.derived_from.as_ref() {
            let dpath = dpath.to_string();
            path = derive_peripheral(&mut p, &dpath, &index)?;
        }
        let path = path.unwrap_or_else(|| RegisterPath::new(&p.name));
        if let Some(regs) = p.registers.as_mut() {
            for rc in take(regs) {
                expand_register_cluster(regs, rc, &path, &index)?;
            }
        }
        match p {
            Peripheral::Single(_) => {
                device.peripherals.push(p);
            }
            Peripheral::Array(info, dim) => {
                for px in names(&info, &dim)
                    .zip(peripheral::base_addresses(&info, &dim))
                    .map(|(name, base_address)| {
                        let mut info = info.clone();
                        info.name = name;
                        info.base_address = base_address;
                        Peripheral::Single(info)
                    })
                {
                    device.peripherals.push(px);
                }
            }
        }
    }

    Ok(device)
}

/// Takes register `size`, `access`, `reset_value` and `reset_mask`
/// from peripheral or device properties if absent in register
pub fn expand_properties(device: &mut Device) {
    let default = device.default_register_properties;
    for p in &mut device.peripherals {
        if p.derived_from.is_some() {
            continue;
        }
        let default = p.default_register_properties.derive_from(&default);
        if let Some(regs) = p.registers.as_mut() {
            expand_properties_registers(regs, &default);
        }
    }
}

fn expand_properties_registers(regs: &mut [RegisterCluster], default: &RegisterProperties) {
    for rc in regs {
        match rc {
            RegisterCluster::Cluster(c) => {
                if c.derived_from.is_some() {
                    continue;
                }
                let default = c.default_register_properties.derive_from(default);
                expand_properties_registers(&mut c.children, &default);
            }
            RegisterCluster::Register(r) => {
                if r.derived_from.is_some() {
                    continue;
                }
                r.properties = r.properties.derive_from(default);
            }
        }
    }
}
