//! Provides [expand] method to convert arrays, clusters and derived items in regular instances

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::mem::take;
use svd_rs::{
    array::names, cluster, field, peripheral, register, BitRange, Cluster, ClusterInfo, DeriveFrom,
    Device, EnumeratedValues, Field, Peripheral, Register, RegisterCluster, RegisterProperties,
};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct RPath {
    peripheral: String,
    path: Vec<String>,
}

impl RPath {
    pub fn new(p: &str) -> Self {
        Self {
            peripheral: p.into(),
            path: Vec::new(),
        }
    }
    pub fn new_child(&self, name: &str) -> Self {
        let mut child = self.clone();
        child.path.push(name.into());
        child
    }
    pub fn split_str(s: &str) -> (Option<Self>, String) {
        Self::split_vec(s.split(".").collect())
    }
    pub fn split_vec(mut v: Vec<&str>) -> (Option<Self>, String) {
        let name = v.pop().unwrap().to_string();
        if v.is_empty() {
            return (None, name);
        } else {
            let mut rpath = Self::new(v[0]);
            rpath.path = v[1..].iter().map(|c| c.to_string()).collect();
            (Some(rpath), name)
        }
    }
    pub fn parent_name(&self) -> (Self, String) {
        let mut parent = self.clone();
        let name = parent.path.pop().unwrap();
        (parent, name)
    }
    pub fn parent(&self) -> Self {
        self.parent_name().0
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct FPath {
    register: RPath,
    field: String,
}

impl FPath {
    pub fn new(r: &RPath, name: &str) -> Self {
        Self {
            register: r.clone(),
            field: name.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Index<'a> {
    peripherals: HashMap<String, &'a Peripheral>,
    clusters: HashMap<(RPath, String), &'a Cluster>,
    registers: HashMap<(RPath, String), &'a Register>,
    fields: HashMap<(RPath, String), &'a Field>,
    evs: HashMap<(FPath, String), &'a EnumeratedValues>,
}

impl<'a> Index<'a> {
    fn add_peripheral(&mut self, p: &'a Peripheral) {
        if let Peripheral::Array(info, dim) = p {
            for name in names(info, dim) {
                let path = RPath::new(&name);
                for r in p.registers() {
                    self.add_register(&path, r);
                }
                for c in p.clusters() {
                    self.add_cluster(&path, c);
                }
                self.peripherals.insert(name, p);
            }
        }
        let path = RPath::new(&p.name);
        for r in p.registers() {
            self.add_register(&path, r);
        }
        for c in p.clusters() {
            self.add_cluster(&path, c);
        }
        self.peripherals.insert(p.name.clone(), p);
    }

    fn add_cluster(&mut self, path: &RPath, c: &'a Cluster) {
        if let Cluster::Array(info, dim) = c {
            for name in names(info, dim) {
                let cpath = RPath::new_child(path, &name);
                for r in c.registers() {
                    self.add_register(&cpath, r);
                }
                for c in c.clusters() {
                    self.add_cluster(&cpath, c);
                }
                self.clusters.insert((path.clone(), name), c);
            }
        }
        let cpath = RPath::new_child(path, &c.name);
        for r in c.registers() {
            self.add_register(&cpath, r);
        }
        for c in c.clusters() {
            self.add_cluster(&cpath, c);
        }
        self.clusters.insert((path.clone(), c.name.to_string()), c);
    }
    fn add_register(&mut self, path: &RPath, r: &'a Register) {
        if let Register::Array(info, dim) = r {
            for name in names(info, dim) {
                let rpath = RPath::new_child(path, &name);
                for f in r.fields() {
                    self.add_field(&rpath, f);
                }
                self.registers.insert((path.clone(), name), r);
            }
        }
        let rpath = RPath::new_child(path, &r.name);
        for f in r.fields() {
            self.add_field(&rpath, f);
        }
        self.registers.insert((path.clone(), r.name.to_string()), r);
    }
    fn add_field(&mut self, path: &RPath, f: &'a Field) {
        if let Field::Array(info, dim) = f {
            for name in names(info, dim) {
                let fpath = FPath::new(path, &name);
                for evs in &f.enumerated_values {
                    if let Some(name) = evs.name.as_ref() {
                        self.evs.insert((fpath.clone(), name.to_string()), evs);
                    }
                }
                self.fields.insert((path.clone(), name), f);
            }
        }
        let fpath = FPath::new(path, &f.name);
        for evs in &f.enumerated_values {
            if let Some(name) = evs.name.as_ref() {
                self.evs.insert((fpath.clone(), name.to_string()), evs);
            }
        }
        self.fields.insert((path.clone(), f.name.to_string()), f);
    }

    pub fn create(device: &'a Device) -> Self {
        let mut index = Self::default();
        for p in &device.peripherals {
            index.add_peripheral(p);
        }
        index
    }

    pub fn get_base_peripheral(&self, path: &str) -> Option<&Peripheral> {
        self.peripherals
            .get(path)
            .and_then(|&p| match &p.derived_from {
                None => Some(p),
                Some(dp) => self.get_base_peripheral(dp),
            })
    }
}

fn expand_register_cluster(
    regs: &mut Vec<RegisterCluster>,
    rc: RegisterCluster,
    path: &RPath,
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
    path: &RPath,
    index: &Index,
) -> Result<()> {
    let mut cpath = None;
    if let Some(dpath) = c.derived_from.as_ref() {
        let (dparent, dname) = RPath::split_str(dpath);
        let d = (match dparent {
            Some(dparent) => {
                if c.children.is_empty() {
                    cpath = Some(RPath::new_child(&dparent, &dname));
                }
                index.clusters.get(&(dparent, dname))
            }
            None => {
                if c.children.is_empty() {
                    cpath = Some(RPath::new_child(path, &dname));
                }
                index.clusters.get(&(path.clone(), dname))
            }
        })
        .ok_or_else(|| anyhow!("cluster {} not found", dpath))?;

        if d.derived_from.is_some() {
            return Err(anyhow!("Multiple derive for {} is not supported", dpath));
        }
        c = c.derive_from(d);
        c.derived_from = None;
    }
    let cpath = cpath.unwrap_or_else(|| RPath::new_child(path, &c.name));

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

fn expand_cluster(regs: &mut Vec<RegisterCluster>, c: ClusterInfo) {
    for rc in c.children {
        match rc {
            RegisterCluster::Cluster(_) => unreachable!(),
            RegisterCluster::Register(mut r) => {
                r.name = format!("{}_{}", c.name, r.name);
                r.address_offset = c.address_offset + r.address_offset;
                regs.push(RegisterCluster::Register(r));
            }
        }
    }
}

fn expand_register_array(
    regs: &mut Vec<RegisterCluster>,
    mut r: Register,
    path: &RPath,
    index: &Index,
) -> Result<()> {
    let mut rpath = None;
    if let Some(dpath) = r.derived_from.as_ref() {
        let (dparent, dname) = RPath::split_str(dpath);
        let d = (match dparent {
            Some(dparent) => {
                if r.fields.is_none() {
                    rpath = Some(RPath::new_child(&dparent, &dname));
                }
                index.registers.get(&(dparent, dname))
            }
            None => {
                if r.fields.is_none() {
                    rpath = Some(RPath::new_child(&path, &dname));
                }
                index.registers.get(&(path.clone(), dname))
            }
        })
        .ok_or_else(|| anyhow!("register {} not found", dpath))?;

        if d.derived_from.is_some() {
            return Err(anyhow!("multiple derive for {} is not supported", dpath));
        }
        r = r.derive_from(d);
        r.derived_from = None;
    }
    let rpath = rpath.unwrap_or_else(|| RPath::new_child(path, &r.name));

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

fn expand_field(fields: &mut Vec<Field>, mut f: Field, rpath: &RPath, index: &Index) -> Result<()> {
    let mut fpath = None;
    if let Some(dpath) = f.derived_from.as_ref() {
        let (dparent, dname) = RPath::split_str(dpath);
        let d = (match dparent {
            Some(dparent) => {
                if f.enumerated_values.is_empty() {
                    fpath = Some(FPath::new(&dparent, &dname));
                }
                index.fields.get(&(dparent, dname))
            }
            None => {
                if f.enumerated_values.is_empty() {
                    fpath = Some(FPath::new(rpath, &dname));
                }
                index.fields.get(&(rpath.clone(), dname))
            }
        })
        .ok_or_else(|| anyhow!("field {} not found", dpath))?;

        if d.derived_from.is_some() {
            return Err(anyhow!("multiple derive for {} is not supported", dpath));
        }
        f = f.derive_from(d);
        f.derived_from = None;
    }
    let fpath = fpath.unwrap_or_else(|| FPath::new(rpath, &f.name));

    for ev in &mut f.enumerated_values {
        derive_enumerated_values(ev, &fpath, index)?;
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

fn derive_enumerated_values(ev: &mut EnumeratedValues, fpath: &FPath, index: &Index) -> Result<()> {
    if let Some(dpath) = ev.derived_from.as_ref() {
        let mut v: Vec<&str> = dpath.split(".").collect();
        let dname = v.pop().unwrap().to_string();
        let d = if v.is_empty() {
            // Only EVNAME: Must be in one of fields in same register
            if let Some(r) = index.registers.get(&fpath.register.parent_name()) {
                let mut found = None;
                'outer: for f in r.fields() {
                    for e in &f.enumerated_values {
                        if e.name.as_deref() == Some(dpath) {
                            found = Some(e);
                            break 'outer;
                        }
                    }
                }
                found
            } else {
                None
            }
        } else {
            let fdname = v.pop().unwrap().to_string();
            if v.is_empty() {
                // FIELD.EVNAME
                index
                    .evs
                    .get(&(FPath::new(&fpath.register, &fdname), dname))
                    .copied()
            } else {
                let (rdpath, rdname) = RPath::split_vec(v);
                let rdpath = if let Some(rdpath) = rdpath {
                    // FULL.PATH.EVNAME:
                    rdpath
                } else {
                    // REG.FIELD.EVNAME
                    let mut rdpath = fpath.register.parent();
                    rdpath.path.push(rdname.into());
                    rdpath
                };
                index
                    .evs
                    .get(&(FPath::new(&rdpath, &fdname), dname))
                    .copied()
            }
        };

        if let Some(d) = d {
            if d.derived_from.is_some() {
                return Err(anyhow!("multiple derive for {} is not supported", dpath));
            }
            *ev = ev.derive_from(d);
            ev.derived_from = None;
        } else {
            return Err(anyhow!(
                "enumeratedValues {} not found, parent field: {:?}",
                dpath,
                fpath,
            ));
        }
    }
    Ok(())
}

/// Creates clone of device with expanded arrays of peripherals, clusters, registers and fields.
/// Also resolves all `derivedFrom` reference pathes
pub fn expand(indevice: &Device) -> Result<Device> {
    let mut device = indevice.clone();

    let index = Index::create(&indevice);

    let peripherals = take(&mut device.peripherals);
    for mut p in peripherals {
        let mut path = None;
        if let Some(dpath) = p.derived_from.as_ref() {
            if let Some(d) = index.get_base_peripheral(dpath) {
                if p.registers.is_none() {
                    path = Some(RPath::new(dpath));
                }
                p = p.derive_from(d);
                p.derived_from = None;
            } else {
                return Err(anyhow!("peripheral {} not found", dpath));
            }
        }
        let path = path.unwrap_or_else(|| RPath::new(&p.name));
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
    let default = device.default_register_properties.clone();
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
                let default = c.default_register_properties.derive_from(&default);
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
