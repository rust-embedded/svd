//! Provides [expand] method to convert arrays, clusters and derived items in regular instances

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::mem::take;
use svd_rs::{
    array::names, cluster, field, peripheral, register, BitRange, Cluster, ClusterInfo, DeriveFrom,
    Device, EnumeratedValues, Field, Peripheral, Register, RegisterCluster, RegisterProperties,
};

#[derive(Clone, Debug, Default)]
struct Index<'a> {
    peripherals: HashMap<String, &'a Peripheral>,
    clusters: HashMap<String, &'a Cluster>,
    registers: HashMap<String, &'a Register>,
    fields: HashMap<String, &'a Field>,
    evs: HashMap<String, &'a EnumeratedValues>,
}

impl<'a> Index<'a> {
    fn add_peripheral(&mut self, p: &'a Peripheral) {
        let path = &p.name;
        if let Peripheral::Array(info, dim) = p {
            for name in names(info, dim) {
                let path = name;
                for r in p.registers() {
                    self.add_register(&path, r);
                }
                for c in p.clusters() {
                    self.add_cluster(&path, c);
                }
                self.peripherals.insert(path, p);
            }
        }
        for r in p.registers() {
            self.add_register(path, r);
        }
        for c in p.clusters() {
            self.add_cluster(path, c);
        }
        self.peripherals.insert(path.into(), p);
    }

    fn add_cluster(&mut self, path: &str, c: &'a Cluster) {
        if let Cluster::Array(info, dim) = c {
            for name in names(info, dim) {
                let cpath = format!("{}.{}", path, name);
                for r in c.registers() {
                    self.add_register(&cpath, r);
                }
                for c in c.clusters() {
                    self.add_cluster(&cpath, c);
                }
                self.clusters.insert(cpath, c);
            }
        }
        let cpath = format!("{}.{}", path, c.name);
        for r in c.registers() {
            self.add_register(&cpath, r);
        }
        for c in c.clusters() {
            self.add_cluster(&cpath, c);
        }
        self.clusters.insert(cpath, c);
    }
    fn add_register(&mut self, path: &str, r: &'a Register) {
        if let Register::Array(info, dim) = r {
            for name in names(info, dim) {
                let rpath = format!("{}.{}", path, name);
                for f in r.fields() {
                    self.add_field(&rpath, f);
                }
                self.registers.insert(rpath, r);
            }
        }
        let rpath = format!("{}.{}", path, r.name);
        for f in r.fields() {
            self.add_field(&rpath, f);
        }
        self.registers.insert(rpath, r);
    }
    fn add_field(&mut self, path: &str, f: &'a Field) {
        if let Field::Array(info, dim) = f {
            for name in names(info, dim) {
                let fpath = format!("{}.{}", path, name);
                for evs in &f.enumerated_values {
                    if let Some(name) = evs.name.as_ref() {
                        let epath = format!("{}.{}", fpath, name);
                        self.evs.insert(epath, evs);
                    }
                }
                self.fields.insert(fpath, f);
            }
        }
        let fpath = format!("{}.{}", path, f.name);
        for evs in &f.enumerated_values {
            if let Some(name) = evs.name.as_ref() {
                let epath = format!("{}.{}", fpath, name);
                self.evs.insert(epath, evs);
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
    path: &str,
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
    path: &str,
    index: &Index,
) -> Result<()> {
    let cpath;
    if let Some(dpath) = c.derived_from.as_ref() {
        let d = index
            .clusters
            .get(dpath)
            .or_else(|| index.clusters.get(&format!("{}.{}", path, dpath)))
            .ok_or_else(|| anyhow!("Cluster {} not found", dpath))?;
        cpath = if c.children.is_empty() {
            dpath.to_string()
        } else {
            format!("{}.{}", path, c.name)
        };
        if d.derived_from.is_some() {
            return Err(anyhow!("Multiple derive for {} is not supported", dpath));
        }
        c = c.derive_from(d);
        c.derived_from = None;
    } else {
        cpath = format!("{}.{}", path, c.name);
    };

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
    path: &str,
    index: &Index,
) -> Result<()> {
    let rpath;
    if let Some(dpath) = r.derived_from.as_ref() {
        let d = index
            .registers
            .get(dpath)
            .or_else(|| index.registers.get(&format!("{}.{}", path, dpath)))
            .ok_or_else(|| anyhow!("register {} not found", dpath))?;
        rpath = if r.fields.is_none() {
            dpath.to_string()
        } else {
            format!("{}.{}", path, r.name)
        };
        if d.derived_from.is_some() {
            return Err(anyhow!("multiple derive for {} is not supported", dpath));
        }
        r = r.derive_from(d);
        r.derived_from = None;
    } else {
        rpath = format!("{}.{}", path, r.name);
    };

    if let Some(field) = r.fields.as_mut() {
        for f in take(field) {
            expand_field(field, f, path, &rpath, index)?;
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
    regparent: &str,
    rpath: &str,
    index: &Index,
) -> Result<()> {
    let fpath;
    if let Some(dpath) = f.derived_from.as_ref() {
        let d = index
            .fields
            .get(dpath)
            .or_else(|| index.fields.get(&format!("{}.{}", rpath, dpath)))
            .ok_or_else(|| anyhow!("field {} not found", dpath))?;
        fpath = if f.enumerated_values.is_empty() {
            dpath.to_string()
        } else {
            format!("{}.{}", rpath, f.name)
        };
        if d.derived_from.is_some() {
            return Err(anyhow!("multiple derive for {} is not supported", dpath));
        }
        f = f.derive_from(d);
        f.derived_from = None;
    } else {
        fpath = format!("{}.{}", rpath, f.name);
    };

    for ev in &mut f.enumerated_values {
        derive_enumerated_values(ev, regparent, rpath, &fpath, index)?;
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
    regparent: &str,
    rpath: &str,
    fpath: &str,
    index: &Index,
) -> Result<()> {
    if let Some(dpath) = ev.derived_from.as_ref() {
        let d = match dpath.chars().filter(|&c| c == '.').count() {
            // Only EVNAME: Must be in one of fields in same register
            0 => {
                if let Some(r) = index.registers.get(rpath) {
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
            }
            // FIELD.EVNAME: Search in same field
            1 => index.evs.get(&format!("{}.{}", rpath, dpath)).copied(),
            // FULL.PATH.EVNAME:
            2 => index.evs.get(&format!("{}.{}", regparent, dpath)).copied(),
            _ => index.evs.get(dpath).copied(),
        };
        if let Some(d) = d {
            if d.derived_from.is_some() {
                return Err(anyhow!("multiple derive for {} is not supported", dpath));
            }
            *ev = ev.derive_from(d);
            ev.derived_from = None;
        } else {
            return Err(anyhow!(
                "enumeratedValues {} not found, parent field: {}, regparent: {}",
                dpath,
                fpath,
                regparent,
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
        let path;
        if let Some(dpath) = p.derived_from.as_ref() {
            if let Some(d) = index.get_base_peripheral(dpath) {
                path = if p.registers.is_none() {
                    dpath.to_string()
                } else {
                    p.name.to_string()
                };
                p = p.derive_from(d);
                p.derived_from = None;
            } else {
                return Err(anyhow!("peripheral {} not found", dpath));
            }
        } else {
            path = p.name.to_string();
        }
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
