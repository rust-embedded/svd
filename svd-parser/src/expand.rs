//! Provides [expand] method to convert arrays, clusters and derived items in regular instances

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt;
use std::mem::take;
use svd_rs::{
    array::{descriptions, names},
    cluster, field, peripheral, register, BitRange, Cluster, ClusterInfo, DeriveFrom, Device,
    EnumeratedValues, Field, Peripheral, Register, RegisterCluster, RegisterProperties,
};

/// Path to `peripheral` or `cluster` element
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct BlockPath {
    pub peripheral: String,
    pub path: Vec<String>,
}

impl BlockPath {
    pub fn new(p: impl Into<String>) -> Self {
        Self {
            peripheral: p.into(),
            path: Vec::new(),
        }
    }
    pub fn new_cluster(&self, name: impl Into<String>) -> Self {
        let mut child = self.clone();
        child.path.push(name.into());
        child
    }
    pub fn new_register(&self, name: impl Into<String>) -> RegisterPath {
        RegisterPath::new(self.clone(), name)
    }
    pub fn parse_str(s: &str) -> (Option<Self>, &str) {
        Self::parse_vec(s.split('.').collect())
    }
    pub fn parse_vec(mut v: Vec<&str>) -> (Option<Self>, &str) {
        let name = v.pop().unwrap();
        let mut iter = v.into_iter();
        let block = if let Some(p) = iter.next() {
            let mut path = Self::new(p);
            path.path = iter.map(Into::into).collect();
            Some(path)
        } else {
            None
        };
        (block, name)
    }
    pub fn name(&self) -> &String {
        self.path.last().unwrap()
    }
    pub fn parent(&self) -> Option<Self> {
        let mut p = self.clone();
        p.path.pop()?;
        Some(p)
    }
}

impl fmt::Display for BlockPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.peripheral)?;
        for p in &self.path {
            f.write_str(".")?;
            f.write_str(p)?;
        }
        Ok(())
    }
}

/// Path to `register` element
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct RegisterPath {
    pub block: BlockPath,
    pub name: String,
}

impl RegisterPath {
    pub fn new(block: BlockPath, name: impl Into<String>) -> Self {
        Self {
            block,
            name: name.into(),
        }
    }
    pub fn new_field(&self, name: impl Into<String>) -> FieldPath {
        FieldPath::new(self.clone(), name)
    }
    pub fn parse_str(s: &str) -> (Option<BlockPath>, &str) {
        BlockPath::parse_str(s)
    }
    pub fn parse_vec(v: Vec<&str>) -> (Option<BlockPath>, &str) {
        BlockPath::parse_vec(v)
    }
    pub fn peripheral(&self) -> &String {
        &self.block.peripheral
    }
}

impl fmt::Display for RegisterPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.block.fmt(f)?;
        f.write_str(".")?;
        f.write_str(&self.name)?;
        Ok(())
    }
}

/// Path to `field` element
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct FieldPath {
    pub register: RegisterPath,
    pub name: String,
}

impl FieldPath {
    pub fn new(register: RegisterPath, name: impl Into<String>) -> Self {
        Self {
            register,
            name: name.into(),
        }
    }
    pub fn new_enum(&self, name: impl Into<String>) -> EnumPath {
        EnumPath::new(self.clone(), name)
    }
    pub fn parse_str(s: &str) -> (Option<RegisterPath>, &str) {
        Self::parse_vec(s.split('.').collect())
    }
    pub fn parse_vec(mut v: Vec<&str>) -> (Option<RegisterPath>, &str) {
        let name = v.pop().unwrap();
        let register = if !v.is_empty() {
            let (block, rname) = RegisterPath::parse_vec(v);
            Some(RegisterPath::new(block.unwrap(), rname))
        } else {
            None
        };
        (register, name)
    }
    pub fn register(&self) -> &RegisterPath {
        &self.register
    }
    pub fn peripheral(&self) -> &String {
        self.register.peripheral()
    }
}

impl fmt::Display for FieldPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.register.fmt(f)?;
        f.write_str(".")?;
        f.write_str(&self.name)?;
        Ok(())
    }
}

/// Path to `enumeratedValues` element
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct EnumPath {
    pub field: FieldPath,
    pub name: String,
}

impl EnumPath {
    pub fn new(field: FieldPath, name: impl Into<String>) -> Self {
        Self {
            field,
            name: name.into(),
        }
    }
    pub fn field(&self) -> &FieldPath {
        &self.field
    }
    pub fn register(&self) -> &RegisterPath {
        &self.field.register
    }
    pub fn peripheral(&self) -> &String {
        self.field.peripheral()
    }
}

impl fmt::Display for EnumPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.field.fmt(f)?;
        f.write_str(".")?;
        f.write_str(&self.name)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Index<'a> {
    pub peripherals: HashMap<BlockPath, &'a Peripheral>,
    pub clusters: HashMap<BlockPath, &'a Cluster>,
    pub registers: HashMap<RegisterPath, &'a Register>,
    pub fields: HashMap<FieldPath, &'a Field>,
    pub evs: HashMap<EnumPath, &'a EnumeratedValues>,
}

impl<'a> Index<'a> {
    fn add_peripheral(&mut self, p: &'a Peripheral) {
        if let Peripheral::Array(info, dim) = p {
            for name in names(info, dim) {
                let path = BlockPath::new(name);
                for r in p.registers() {
                    self.add_register(&path, r);
                }
                for c in p.clusters() {
                    self.add_cluster(&path, c);
                }
                self.peripherals.insert(path, p);
            }
        }
        let path = BlockPath::new(&p.name);
        for r in p.registers() {
            self.add_register(&path, r);
        }
        for c in p.clusters() {
            self.add_cluster(&path, c);
        }
        self.peripherals.insert(path, p);
    }

    fn add_cluster(&mut self, path: &BlockPath, c: &'a Cluster) {
        if let Cluster::Array(info, dim) = c {
            for name in names(info, dim) {
                let cpath = path.new_cluster(name);
                for r in c.registers() {
                    self.add_register(&cpath, r);
                }
                for c in c.clusters() {
                    self.add_cluster(&cpath, c);
                }
                self.clusters.insert(cpath, c);
            }
        }
        let cpath = path.new_cluster(&c.name);
        for r in c.registers() {
            self.add_register(&cpath, r);
        }
        for c in c.clusters() {
            self.add_cluster(&cpath, c);
        }
        self.clusters.insert(cpath, c);
    }
    fn add_register(&mut self, path: &BlockPath, r: &'a Register) {
        if let Register::Array(info, dim) = r {
            for name in names(info, dim) {
                let rpath = path.new_register(name);
                for f in r.fields() {
                    self.add_field(&rpath, f);
                }
                self.registers.insert(rpath, r);
            }
        }
        let rpath = path.new_register(&r.name);
        for f in r.fields() {
            self.add_field(&rpath, f);
        }
        self.registers.insert(rpath, r);
    }
    fn add_field(&mut self, path: &RegisterPath, f: &'a Field) {
        if let Field::Array(info, dim) = f {
            for name in names(info, dim) {
                let fpath = path.new_field(name);
                for evs in &f.enumerated_values {
                    if let Some(name) = evs.name.as_ref() {
                        self.evs.insert(fpath.new_enum(name), evs);
                    }
                }
                self.fields.insert(fpath, f);
            }
        }
        let fpath = path.new_field(&f.name);
        for evs in &f.enumerated_values {
            if let Some(name) = evs.name.as_ref() {
                self.evs.insert(fpath.new_enum(name), evs);
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
    path: &BlockPath,
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
    path: &BlockPath,
    index: &Index,
) -> Result<()> {
    let mut cpath = None;
    let dpath = c.derived_from.take();
    if let Some(dpath) = dpath {
        cpath = derive_cluster(&mut c, &dpath, path, index)?;
    }
    let cpath = cpath.unwrap_or_else(|| path.new_cluster(&c.name));

    for rc in take(&mut c.children) {
        expand_register_cluster(&mut c.children, rc, &cpath, index)?;
    }

    match c {
        Cluster::Single(c) => expand_cluster(regs, c),
        Cluster::Array(info, dim) => {
            for c in names(&info, &dim)
                .zip(descriptions(&info, &dim))
                .zip(cluster::address_offsets(&info, &dim))
                .map(|((name, description), address_offset)| {
                    let mut info = info.clone();
                    info.name = name;
                    info.description = description;
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

pub fn derive_cluster(
    c: &mut Cluster,
    dpath: &str,
    path: &BlockPath,
    index: &Index,
) -> Result<Option<BlockPath>> {
    let (dparent, dname) = BlockPath::parse_str(dpath);
    let rdpath;
    let cluster_path;
    let d = (if let Some(dparent) = dparent {
        cluster_path = dparent.new_cluster(dname);
        rdpath = dparent;
        index.clusters.get(&cluster_path)
    } else {
        cluster_path = path.new_cluster(dname);
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

pub fn derive_register(
    r: &mut Register,
    dpath: &str,
    path: &BlockPath,
    index: &Index,
) -> Result<Option<RegisterPath>> {
    let (dblock, dname) = RegisterPath::parse_str(dpath);
    let rdpath;
    let reg_path;
    let d = (if let Some(dblock) = dblock {
        reg_path = dblock.new_register(dname);
        rdpath = dblock;
        index.registers.get(&reg_path)
    } else {
        reg_path = path.new_register(dname);
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

pub fn derive_field(
    f: &mut Field,
    dpath: &str,
    rpath: &RegisterPath,
    index: &Index,
) -> Result<Option<FieldPath>> {
    let (dregister, dname) = FieldPath::parse_str(dpath);
    let rdpath;
    let field_path;
    let d = (if let Some(dregister) = dregister {
        field_path = dregister.new_field(dname);
        rdpath = dregister;
        index.fields.get(&field_path)
    } else {
        field_path = rpath.new_field(dname);
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
                regs.push(r.into());
            }
        }
    }
}

fn expand_register_array(
    regs: &mut Vec<RegisterCluster>,
    mut r: Register,
    path: &BlockPath,
    index: &Index,
) -> Result<()> {
    let mut rpath = None;
    let dpath = r.derived_from.take();
    if let Some(dpath) = dpath {
        rpath = derive_register(&mut r, &dpath, path, index)?;
    }
    let rpath = rpath.unwrap_or_else(|| path.new_register(&r.name));

    if let Some(field) = r.fields.as_mut() {
        for f in take(field) {
            expand_field(field, f, &rpath, index)?;
        }
    }

    match r {
        Register::Single(_) => {
            regs.push(r.into());
        }
        Register::Array(info, dim) => {
            for rx in names(&info, &dim)
                .zip(descriptions(&info, &dim))
                .zip(register::address_offsets(&info, &dim))
                .map(|((name, description), address_offset)| {
                    let mut info = info.clone();
                    info.name = name;
                    info.description = description;
                    info.address_offset = address_offset;
                    info.single()
                })
            {
                regs.push(rx.into());
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
    let dpath = f.derived_from.take();
    if let Some(dpath) = dpath {
        fpath = derive_field(&mut f, &dpath, rpath, index)?;
    }
    let fpath = fpath.unwrap_or_else(|| rpath.new_field(&f.name));

    for ev in &mut f.enumerated_values {
        let dpath = ev.derived_from.take();
        if let Some(dpath) = dpath {
            derive_enumerated_values(ev, &dpath, &fpath, index)?;
        }
    }

    match f {
        Field::Single(_) => {
            fields.push(f);
        }
        Field::Array(info, dim) => {
            for fx in names(&info, &dim)
                .zip(descriptions(&info, &dim))
                .zip(field::bit_offsets(&info, &dim))
                .map(|((name, description), bit_offset)| {
                    let mut info = info.clone();
                    info.name = name;
                    info.description = description;
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

pub fn derive_enumerated_values(
    ev: &mut EnumeratedValues,
    dpath: &str,
    fpath: &FieldPath,
    index: &Index,
) -> Result<EnumPath> {
    let mut v: Vec<&str> = dpath.split('.').collect();
    let dname = v.pop().unwrap();
    let d = if v.is_empty() {
        // Only EVNAME: Must be in one of fields in same register
        let rdpath = &fpath.register;
        if let Some(r) = index.registers.get(rdpath) {
            let mut found = None;
            for f in r.fields() {
                let epath = EnumPath::new(rdpath.new_field(&f.name), dname);
                if let Some(d) = index.evs.get(&epath) {
                    found = Some((d, epath));
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
            fpath.register.new_field(fdname)
        } else {
            let (rdpath, rdname) = RegisterPath::parse_vec(v);
            let rdpath = if let Some(rdpath) = rdpath {
                // FULL.PATH.EVNAME:
                rdpath.new_register(rdname)
            } else {
                // REG.FIELD.EVNAME
                fpath.register.block.new_register(rdname)
            };
            FieldPath::new(rdpath, fdname)
        };
        let epath = EnumPath::new(fdpath, dname);
        index.evs.get(&epath).map(|d| (d, epath))
    };

    if let Some((d, epath)) = d {
        *ev = ev.derive_from(d);
        if let Some(dpath) = d.derived_from.as_ref() {
            derive_enumerated_values(ev, dpath, &epath.field, index)
        } else {
            Ok(epath)
        }
    } else {
        Err(anyhow!(
            "enumeratedValues {} not found, parent field: {:?}",
            dpath,
            fpath,
        ))
    }
}

pub fn derive_peripheral(
    p: &mut Peripheral,
    dpath: &str,
    index: &Index,
) -> Result<Option<BlockPath>> {
    let mut path = None;
    let derpath = BlockPath::new(dpath);
    let d = index
        .peripherals
        .get(&derpath)
        .ok_or_else(|| anyhow!("peripheral {} not found", dpath))?;
    if p.registers.is_none() {
        path = Some(derpath);
    }
    *p = p.derive_from(d);
    if let Some(dpath) = d.derived_from.as_ref() {
        path = derive_peripheral(p, dpath, index)?;
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
        let dpath = p.derived_from.take();
        if let Some(dpath) = dpath {
            path = derive_peripheral(&mut p, &dpath, &index)?;
        }
        let path = path.unwrap_or_else(|| BlockPath::new(&p.name));
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
                    .zip(descriptions(&info, &dim))
                    .zip(peripheral::base_addresses(&info, &dim))
                    .map(|((name, description), base_address)| {
                        let mut info = info.clone();
                        info.name = name;
                        info.description = description;
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
