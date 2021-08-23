use globset::Glob;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use svd_parser::svd::{
    self, clusterinfo::ClusterInfoBuilder, fieldinfo::FieldInfoBuilder,
    peripheral::PeripheralBuilder, registerinfo::RegisterInfoBuilder, Access, AddressBlock,
    AddressBlockUsage, BitRange, Cluster, ClusterInfo, Cpu, Device, DimElement, Endian,
    EnumeratedValue, EnumeratedValues, Field, FieldInfo, Interrupt, Peripheral, Register,
    RegisterCluster, RegisterInfo, RegisterProperties, Usage, ValidateLevel, WriteConstraint,
    WriteConstraintRange,
};
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

const VAL_LVL: ValidateLevel = ValidateLevel::Weak;

trait AsTypeMut {
    fn as_hash_mut(&mut self) -> Option<&mut Hash>;
}

impl AsTypeMut for Yaml {
    fn as_hash_mut(&mut self) -> Option<&mut Hash> {
        match self {
            Yaml::Hash(h) => Some(h),
            _ => None,
        }
    }
}

trait ToYaml {
    fn to_yaml(self) -> Yaml;
}

impl ToYaml for &str {
    fn to_yaml(self) -> Yaml {
        Yaml::String(self.into())
    }
}

impl ToYaml for Yaml {
    fn to_yaml(self) -> Yaml {
        self
    }
}

fn parse_i64(val: &Yaml) -> Option<i64> {
    match val {
        Yaml::Integer(i) => Some(*i),
        Yaml::String(text) => {
            let text = text.replace("_", "");
            (if text.starts_with("0x") || text.starts_with("0X") {
                i64::from_str_radix(&text["0x".len()..], 16)
            } else if text.starts_with('#') {
                // Handle strings in the binary form of:
                // #01101x1
                // along with don't care character x (replaced with 0)
                i64::from_str_radix(
                    &str::replace(&text.to_lowercase()["#".len()..], "x", "0"),
                    2,
                )
            } else if text.starts_with("0b") {
                // Handle strings in the binary form of:
                // 0b01101x1
                // along with don't care character x (replaced with 0)
                i64::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2)
            } else {
                text.parse::<i64>()
            })
            .ok()
        }
        _ => None,
    }
}

fn parse_bool(val: &Yaml) -> Option<bool> {
    match val {
        Yaml::Boolean(b) => Some(*b),
        Yaml::Integer(i) => match *i {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        },
        Yaml::String(text) => match text.as_str() {
            "true" | "True" => Some(true),
            "false" | "False" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

trait GetVal {
    fn get_bool<K: ToYaml>(&self, k: K) -> Option<bool>;
    fn get_i64<K: ToYaml>(&self, k: K) -> Option<i64>;
    fn get_str<K: ToYaml>(&self, k: K) -> Option<&str>;
    fn get_hash<K: ToYaml>(&self, k: K) -> Option<&Hash>;
    fn get_vec<K: ToYaml>(&self, k: K) -> Option<&Vec<Yaml>>;
}

impl GetVal for Hash {
    fn get_bool<K: ToYaml>(&self, k: K) -> Option<bool> {
        self.get(&k.to_yaml()).and_then(parse_bool)
    }
    fn get_i64<K: ToYaml>(&self, k: K) -> Option<i64> {
        self.get(&k.to_yaml()).and_then(parse_i64)
    }
    fn get_str<K: ToYaml>(&self, k: K) -> Option<&str> {
        self.get(&k.to_yaml()).and_then(Yaml::as_str)
    }
    fn get_hash<K: ToYaml>(&self, k: K) -> Option<&Hash> {
        self.get(&k.to_yaml()).and_then(Yaml::as_hash)
    }
    fn get_vec<K: ToYaml>(&self, k: K) -> Option<&Vec<Yaml>> {
        self.get(&k.to_yaml()).and_then(Yaml::as_vec)
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .unwrap_or_else(|| panic!("Please, specify yaml file path as first argument"));
    process_file(Path::new(path))
}

fn process_file(yaml_file: &Path) -> std::io::Result<()> {
    // Load the specified YAML root file
    let f = File::open(yaml_file)?;
    let mut contents = String::new();
    (&f).read_to_string(&mut contents)?;
    let mut docs = YamlLoader::load_from_str(&contents).unwrap();
    let root = docs[0].as_hash_mut().unwrap(); // select the first document
    root.insert("_path".to_yaml(), yaml_file.to_str().unwrap().to_yaml());

    // Load the specified SVD file
    let svdpath = abspath(
        &yaml_file,
        &Path::new(
            root.get_str("_svd")
                .unwrap_or_else(|| panic!("You must have an svd key in the root YAML file")),
        ),
    );
    let mut svdpath_out = svdpath.clone();
    svdpath_out.set_extension("svd.patched");
    let f = File::open(svdpath)?;
    let mut contents = String::new();
    (&f).read_to_string(&mut contents)?;
    let mut svd = svd_parser::parse_with_config(
        &contents,
        &svd_parser::Config {
            validate_level: ValidateLevel::Disabled,
        },
    )
    .expect("Failed to parse input SVD");

    // Load all included YAML files
    yaml_includes(root);

    // Process device
    svd.process(root, true);

    // SVD should now be updated, write it out
    let svd_out = svd_encoder::encode(&svd).expect("Encode failed");

    let mut f = File::create(&svdpath_out)?;
    f.write(svd_out.as_bytes()).unwrap();

    Ok(())
}

/// Gets the absolute path of relpath from the point of view of frompath.
fn abspath(frompath: &Path, relpath: &Path) -> PathBuf {
    std::fs::canonicalize(frompath.parent().unwrap().join(relpath)).unwrap()
}

/// Recursively loads any included YAML files.
pub fn yaml_includes(parent: &mut Hash) -> Vec<PathBuf> {
    let y_path = "_path".to_yaml();
    let mut included = vec![];
    let self_path = PathBuf::from(parent.get(&y_path).unwrap().as_str().unwrap());
    let inc = parent.get_vec("_include").unwrap_or(&Vec::new()).clone();
    for relpath in inc {
        let path = abspath(&self_path, Path::new(relpath.as_str().unwrap()));
        if included.contains(&path) {
            continue;
        }
        let f = File::open(&path).unwrap();
        let mut contents = String::new();
        (&f).read_to_string(&mut contents).unwrap();
        let mut docs = YamlLoader::load_from_str(&contents).unwrap();
        if docs.is_empty() {
            continue;
        }
        let child = docs[0].as_hash_mut().unwrap();
        let ypath = path.to_str().unwrap().to_yaml();
        child.insert(y_path.clone(), ypath.clone());
        included.push(path.clone());

        // Process any peripheral-level includes in child
        for (pspec, val) in child.iter_mut() {
            if !pspec.as_str().unwrap().starts_with("_") {
                match val {
                    Yaml::Hash(val) if val.contains_key(&"_include".to_yaml()) => {
                        val.insert(y_path.clone(), ypath.clone());
                        included.extend(yaml_includes(val));
                    }
                    _ => {}
                }
            }
        }

        // Process any top-level includes in child
        included.extend(yaml_includes(child));
        update_dict(parent, child);
    }
    included
}

/// Recursively merge child.key into parent.key, with parent overriding
fn update_dict(parent: &mut Hash, child: &Hash) {
    use linked_hash_map::Entry;
    for (key, val) in child.iter() {
        match key {
            Yaml::String(key) if key == "_path" || key == "_include" => continue,
            key if parent.contains_key(key) => {
                if let Entry::Occupied(mut e) = parent.entry(key.clone()) {
                    match e.get_mut() {
                        el if el == val => {
                            println!("In {:?}: dublicate rule {:?}, ignored", key, val);
                        }
                        Yaml::Array(a) => match val {
                            Yaml::Array(val) => {
                                a.extend(val.clone());
                            }
                            Yaml::String(_) => {
                                if !a.contains(val) {
                                    a.push(val.clone());
                                } else {
                                    println!("In {:?}: dublicate rule {:?}, ignored", key, val);
                                }
                            }
                            _ => {}
                        },
                        Yaml::Hash(h) => {
                            update_dict(h, val.as_hash().unwrap());
                        }
                        s if matches!(s, Yaml::String(_)) => match val {
                            Yaml::Array(a) => {
                                if !a.contains(s) {
                                    let mut a = a.clone();
                                    a.insert(0, s.clone());
                                    e.insert(Yaml::Array(a));
                                } else {
                                    println!("In {:?}: dublicate rule {:?}, ignored", key, s);
                                }
                            }
                            s2 if matches!(s2, Yaml::String(_)) => {
                                println!(
                                    "In {:?}: conflicting rules {:?} and {:?}, ignored",
                                    key, s, s2
                                );
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            _ => {
                parent.insert(key.clone(), val.clone());
            }
        }
    }
}

/// Check if name matches against a specification
fn matchname(name: &str, spec: &str) -> bool {
    if spec.starts_with("_") {
        return false;
    }
    for subspec in spec.split(",") {
        let glob = Glob::new(subspec).unwrap().compile_matcher();
        if glob.is_match(name) {
            return true;
        }
    }
    false
}

/// If a name matches a specification, return the first sub-specification that it matches
fn matchsubspec<'a>(name: &str, spec: &'a str) -> Option<&'a str> {
    if !matchname(name, spec) {
        return None;
    }
    for subspec in spec.split(",") {
        let glob = Glob::new(subspec).unwrap().compile_matcher();
        if glob.is_match(name) {
            return Some(subspec);
        }
    }
    return None;
}

fn modify_register_properties(p: &mut RegisterProperties, f: &str, val: &Yaml) {
    match f {
        "size" => p.size = parse_i64(val).map(|v| v as u32),
        "access" => p.access = val.as_str().and_then(Access::from_str),
        "resetValue" => p.reset_value = parse_i64(val).map(|v| v as u64),
        "resetMask" => p.reset_mask = parse_i64(val).map(|v| v as u64),
        "protection" => {}
        _ => {}
    }
}

fn get_register_properties(h: &Hash) -> RegisterProperties {
    RegisterProperties::builder()
        .size(h.get_i64("size").map(|v| v as u32))
        .access(h.get_str("access").and_then(Access::from_str))
        .reset_value(h.get_i64("resetValue").map(|v| v as u64))
        .reset_mask(h.get_i64("resetMask").map(|v| v as u64))
}

struct PerIter<'a, 'b> {
    it: std::slice::IterMut<'a, Peripheral>,
    spec: &'b str,
    check_derived: bool,
}

impl<'a, 'b> Iterator for PerIter<'a, 'b> {
    type Item = &'a mut Peripheral;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.it.next() {
            if matchname(&next.name, self.spec)
                && !(self.check_derived && next.derived_from.is_some())
            {
                return Some(next);
            }
        }
        None
    }
}

/// Collecting methods for processing device contents
trait DeviceExt {
    /// Iterates over all peripherals that match pspec
    fn iter_peripherals<'a, 'b>(
        &'a mut self,
        spec: &'b str,
        check_derived: bool,
    ) -> PerIter<'a, 'b>;

    /// Work through a device, handling all peripherals
    fn process(&mut self, device: &Hash, update_fields: bool);

    /// Delete registers matched by rspec inside ptag
    fn delete_peripheral(&mut self, pspec: &str);

    /// Create copy of peripheral
    fn copy_peripheral(&mut self, pname: &str, pmod: &Hash, path: &Path);

    /// Modify the `cpu` node inside `device` according to `mod`
    fn modify_cpu(&mut self, cmod: &Hash);

    /// Modify pspec inside device according to pmod
    fn modify_peripheral(&mut self, pspec: &str, pmod: &Hash);

    /// Add pname given by padd to device
    fn add_peripheral(&mut self, pname: &str, padd: &Hash);

    /// Remove registers from pname and mark it as derivedFrom pderive.
    /// Update all derivedFrom referencing pname
    fn derive_peripheral(&mut self, pname: &str, pderive: &str);

    /// Move registers from pold to pnew.
    /// Update all derivedFrom referencing pold
    fn rebase_peripheral(&mut self, pnew: &str, pold: &str);

    /// Work through a peripheral, handling all registers
    fn process_peripheral(&mut self, pspec: &str, peripheral: &Hash, update_fields: bool);
}

impl DeviceExt for Device {
    fn iter_peripherals<'a, 'b>(
        &'a mut self,
        spec: &'b str,
        check_derived: bool,
    ) -> PerIter<'a, 'b> {
        // check_derived=True
        PerIter {
            spec,
            check_derived,
            it: self.peripherals.iter_mut(),
        }
    }

    fn process(&mut self, device: &Hash, update_fields: bool) {
        // Handle any deletions
        match device.get(&"_delete".to_yaml()) {
            Some(Yaml::String(pspec)) => self.delete_peripheral(pspec),
            Some(Yaml::Array(array)) => {
                for pspec in array {
                    self.delete_peripheral(pspec.as_str().unwrap());
                }
            }
            _ => {}
        }

        // Handle any copied peripherals
        for (pname, val) in device.get_hash("_copy").unwrap_or(&Hash::new()) {
            self.copy_peripheral(
                pname.as_str().unwrap(),
                val.as_hash().unwrap(),
                Path::new(device.get_str("_path").unwrap()),
            );
        }

        // Handle any modifications
        for (key, val) in device.get_hash("_modify").unwrap_or(&Hash::new()) {
            let key = key.as_str().unwrap();
            match key {
                "cpu" => self.modify_cpu(val.as_hash().unwrap()),
                "_peripherals" => {
                    for (pspec, pmod) in val.as_hash().unwrap() {
                        self.modify_peripheral(pspec.as_str().unwrap(), pmod.as_hash().unwrap())
                    }
                }
                "vendor" => {
                    todo!()
                }
                "vendorID" => {
                    todo!()
                }
                "name" => self.name = val.as_str().unwrap().into(),
                "series" => {
                    todo!()
                }
                "version" => self.version = val.as_str().map(String::from),
                "description" => self.description = val.as_str().map(String::from),
                "licenseText" => {
                    todo!()
                }
                "headerSystemFilename" => {
                    todo!()
                }
                "headerDefinitionsPrefix" => {
                    todo!()
                }
                "addressUnitBits" => {
                    todo!()
                }
                "width" => self.width = parse_i64(val).map(|v| v as u32),
                "size" | "access" | "protection" | "resetValue" | "resetMask" => {
                    modify_register_properties(&mut self.default_register_properties, key, val)
                }

                _ => self.modify_peripheral(key, val.as_hash().unwrap()),
            }
        }

        // Handle any new peripherals (!)
        for (pname, padd) in device.get_hash("_add").unwrap_or(&Hash::new()) {
            self.add_peripheral(pname.as_str().unwrap(), padd.as_hash().unwrap());
        }

        // Handle any derived peripherals
        for (pname, pderive) in device.get_hash("_derive").unwrap_or(&Hash::new()) {
            self.derive_peripheral(pname.as_str().unwrap(), pderive.as_str().unwrap());
        }

        // Handle any rebased peripherals
        for (pname, pold) in device.get_hash("_rebase").unwrap_or(&Hash::new()) {
            self.rebase_peripheral(pname.as_str().unwrap(), pold.as_str().unwrap());
        }

        // Now process all peripherals
        for (periphspec, val) in device {
            let periphspec = periphspec.as_str().unwrap();
            if !periphspec.starts_with("_") {
                //val["_path"] = device["_path"]; // TODO: check
                self.process_peripheral(periphspec, val.as_hash().unwrap(), update_fields)
            }
        }
    }

    fn delete_peripheral(&mut self, pspec: &str) {
        self.peripherals.retain(|p| !(matchname(&p.name, pspec)));
    }

    fn copy_peripheral(&mut self, pname: &str, pmod: &Hash, path: &Path) {
        let pcopysrc = pmod.get_str("from").unwrap().split(":").collect::<Vec<_>>();
        let mut new = match pcopysrc.as_slice() {
            [ppath, pcopyname] => {
                let f = File::open(abspath(path, &Path::new(ppath))).unwrap();
                let mut contents = String::new();
                (&f).read_to_string(&mut contents).unwrap();
                let filedev = svd_parser::parse(&contents).expect("Failed to parse input SVD");
                filedev
                    .peripherals
                    .iter()
                    .find(|p| &p.name == pcopyname)
                    .unwrap_or_else(|| panic!("peripheral {} not found", pcopyname))
                    .clone()
            }
            [pcopyname] => {
                let mut new = self
                    .peripherals
                    .iter()
                    .find(|p| &p.name == pcopyname)
                    .unwrap_or_else(|| panic!("peripheral {} not found", pcopyname))
                    .clone();
                // When copying from a peripheral in the same file, remove any interrupts.
                new.interrupt = Vec::new();
                new
            }
            _ => panic!(),
        };
        new.name = pname.into();
        if let Some(ptag) = self.peripherals.iter_mut().find(|p| &p.name == pname) {
            new.base_address = ptag.base_address;
            new.interrupt = std::mem::take(&mut ptag.interrupt);
            *ptag = new;
        } else {
            self.peripherals.push(new)
        }
    }

    fn modify_cpu(&mut self, cmod: &Hash) {
        let mut cpu = Cpu::builder();
        if let Some(name) = cmod.get_str("name") {
            cpu = cpu.name(name.into());
        }
        if let Some(revision) = cmod.get_str("revision") {
            cpu = cpu.revision(revision.into());
        }
        if let Some(endian) = cmod.get_str("endian").and_then(Endian::from_str) {
            cpu = cpu.endian(endian);
        }
        if let Some(mpu_present) = cmod.get_bool("mpuPresent") {
            cpu = cpu.mpu_present(mpu_present);
        }
        if let Some(fpu_present) = cmod.get_bool("fpuPresent") {
            cpu = cpu.fpu_present(fpu_present);
        }
        if let Some(nvic_priority_bits) = cmod.get_i64("nvicPrioBits") {
            cpu = cpu.nvic_priority_bits(nvic_priority_bits as u32);
        }
        if let Some(has_vendor_systick) = cmod.get_bool("vendorSystickConfig") {
            cpu = cpu.has_vendor_systick(has_vendor_systick);
        }
        if let Some(c) = self.cpu.as_mut() {
            c.modify_from(cpu, VAL_LVL).unwrap();
        } else {
            self.cpu = Some(cpu.build(VAL_LVL).unwrap());
        }
    }

    fn modify_peripheral(&mut self, pspec: &str, pmod: &Hash) {
        for ptag in self.iter_peripherals(pspec, true) {
            ptag.modify_from(make_peripheral(pmod), VAL_LVL).unwrap();
        }
    }

    fn add_peripheral(&mut self, pname: &str, padd: &Hash) {
        if self.peripherals.iter().find(|p| p.name == pname).is_some() {
            panic!("device already has a peripheral {}", pname);
        }

        self.peripherals.push(
            make_peripheral(padd)
                .name(pname.to_string())
                .build(VAL_LVL)
                .unwrap(),
        );
    }

    fn derive_peripheral(&mut self, pname: &str, pderive: &str) {
        assert!(
            self.peripherals
                .iter()
                .find(|p| &p.name == pderive)
                .is_some(),
            "peripheral {} not found",
            pderive
        );
        self.peripherals
            .iter_mut()
            .find(|p| &p.name == pname)
            .unwrap_or_else(|| panic!("peripheral {} not found", pname))
            .modify_from(
                Peripheral::builder().derived_from(Some(pderive.into())),
                VAL_LVL,
            )
            .unwrap();
        for p in self
            .peripherals
            .iter_mut()
            .filter(|p| p.derived_from.as_deref() == Some(pname))
        {
            p.derived_from = Some(pderive.into());
        }
    }

    fn rebase_peripheral(&mut self, pnew: &str, pold: &str) {
        let old = self
            .peripherals
            .iter_mut()
            .find(|p| &p.name == pold)
            .unwrap_or_else(|| panic!("peripheral {} not found", pold));
        let mut d = std::mem::replace(
            old,
            Peripheral::builder()
                .name(pold.into())
                .base_address(old.base_address)
                .interrupt(old.interrupt.clone())
                .derived_from(Some(pnew.into()))
                .build(VAL_LVL)
                .unwrap(),
        );
        let new = self
            .peripherals
            .iter_mut()
            .find(|p| &p.name == pnew)
            .unwrap_or_else(|| panic!("peripheral {} not found", pnew));
        d.name = new.name.clone();
        d.base_address = new.base_address;
        d.interrupt = new.interrupt.clone();
        *new = d;
        for p in self
            .peripherals
            .iter_mut()
            .filter(|p| p.derived_from.as_deref() == Some(pold))
        {
            p.derived_from = Some(pnew.into());
        }
    }

    fn process_peripheral(&mut self, pspec: &str, peripheral: &Hash, update_fields: bool) {
        // Find all peripherals that match the spec
        let mut pcount = 0;
        for ptag in self.iter_peripherals(pspec, false) {
            pcount += 1;
            ptag.process(peripheral, update_fields);
        }
        if pcount == 0 {
            panic!("Could not find {}", pspec);
        }
    }
}

type ClusterMatchIterMut<'a, 'b> = MatchIterMut<'a, 'b, Cluster, std::slice::IterMut<'a, Cluster>>;
type RegMatchIterMut<'a, 'b> = MatchIterMut<'a, 'b, Register, svd::register::RegIterMut<'a>>;

struct MatchIterMut<'a, 'b, T: 'a, I>
where
    T: 'a + GetName,
    I: Iterator<Item = &'a mut T>,
{
    it: I,
    spec: &'b str,
}

impl<'a, 'b, T, I> Iterator for MatchIterMut<'a, 'b, T, I>
where
    T: 'a + GetName,
    I: Iterator<Item = &'a mut T>,
{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.it.next() {
            if matchname(&next.get_name(), self.spec) {
                return Some(next);
            }
        }
        None
    }
}

trait Matched<'a, T: 'a>
where
    Self: Iterator<Item = &'a mut T> + Sized,
    T: 'a,
{
    fn matched<'b>(self, spec: &'b str) -> MatchIterMut<'a, 'b, T, Self>
    where
        T: GetName;
}

impl<'a, T, I> Matched<'a, T> for I
where
    Self: Iterator<Item = &'a mut T> + Sized,
    T: 'a,
{
    fn matched<'b>(self, spec: &'b str) -> MatchIterMut<'a, 'b, T, Self>
    where
        T: GetName,
    {
        MatchIterMut { it: self, spec }
    }
}

struct OptIterMut<'a, T, I>(Option<I>)
where
    T: 'a,
    I: Iterator<Item = &'a mut T>;

impl<'a, T, I> Iterator for OptIterMut<'a, T, I>
where
    T: 'a,
    I: Iterator<Item = &'a mut T>,
{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().and_then(I::next)
    }
}

trait GetName {
    fn get_name(&self) -> &str;
}
impl GetName for Interrupt {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl GetName for Field {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl GetName for Register {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl GetName for Cluster {
    fn get_name(&self) -> &str {
        &self.name
    }
}

/// Collecting methods for processing peripheral contents
trait PeripheralExt {
    /// Iterates over all registers that match rspec and live inside ptag
    fn iter_registers<'a, 'b>(&'a mut self, spec: &'b str) -> RegMatchIterMut<'a, 'b>;

    /// Iterate over all clusters that match cpsec and live inside ptag
    fn iter_clusters<'a, 'b>(&mut self, spec: &str) -> ClusterMatchIterMut<'a, 'b>;

    /// Iterates over all interrupts matching ispec
    fn iter_interrupts<'a, 'b>(
        &'a mut self,
        spec: &'b str,
    ) -> MatchIterMut<'a, 'b, Interrupt, std::slice::IterMut<'a, Interrupt>>;

    /// Work through a peripheral, handling all registers
    fn process(&mut self, peripheral: &Hash, update_fields: bool);

    /// Delete interrupts matched by ispec
    fn delete_interrupt(&mut self, ispec: &str);

    /// Add iname given by iadd to ptag
    fn add_interrupt(&mut self, iname: &str, iadd: &Hash);

    /// Modify ispec according to imod
    fn modify_interrupt(&mut self, ispec: &str, imod: &Hash);

    /// Delete registers matched by rspec inside ptag
    fn delete_register(&mut self, rspec: &str);

    /// Add rname given by radd to ptag
    fn add_register(&mut self, rname: &str, radd: &Hash);

    /// Add rname given by deriving from rsource to ptag
    fn copy_register(&mut self, rname: &str, rderive: &Hash);

    /// Modify rspec inside ptag according to rmod
    fn modify_register(&mut self, rspec: &str, rmod: &Hash);

    /// Modify cspec inside ptag according to cmod
    fn modify_cluster(&mut self, cspec: &str, cmod: &Hash);
    /// Work through a register, handling all fields
    fn process_register(&mut self, rspec: &str, register: &Hash, update_fields: bool);

    /// Delete substring from register names inside ptag. Strips from the
    /// beginning of the name by default
    fn strip(&mut self, substr: &str, strip_end: bool);

    /// Collect same registers in peripheral into register array
    fn collect_in_array(&mut self, rspec: &str, rmod: &Hash);

    /// Collect registers in peripheral into clusters
    fn collect_in_cluster(&mut self, cname: &str, cmod: &Hash);
}

impl PeripheralExt for Peripheral {
    fn iter_registers<'a, 'b>(&'a mut self, spec: &'b str) -> RegMatchIterMut<'a, 'b> {
        self.reg_iter_mut().matched(spec)
    }

    fn iter_interrupts<'a, 'b>(
        &'a mut self,
        spec: &'b str,
    ) -> MatchIterMut<'a, 'b, Interrupt, std::slice::IterMut<'a, Interrupt>> {
        self.interrupt.iter_mut().matched(spec)
    }

    fn process(&mut self, pmod: &Hash, update_fields: bool) {
        // For derived peripherals, only process interrupts
        if self.derived_from.is_some() {
            if let Some(deletions) = pmod.get_hash("_delete") {
                for (rspec, val) in deletions {
                    if rspec.as_str() == Some("_interrupts") {
                        for ispec in val.as_vec().unwrap() {
                            self.delete_interrupt(ispec.as_str().unwrap())
                        }
                    }
                }
            }
            for (rspec, rmod) in pmod.get_hash("_modify").unwrap_or(&Hash::new()) {
                if rspec.as_str() == Some("_interrupts") {
                    for (ispec, val) in rmod.as_hash().unwrap() {
                        self.modify_interrupt(ispec.as_str().unwrap(), val.as_hash().unwrap())
                    }
                }
            }
            for (rname, radd) in pmod.get_hash("_add").unwrap_or(&Hash::new()) {
                if rname.as_str() == Some("_interrupts") {
                    for (iname, val) in radd.as_hash().unwrap() {
                        self.add_interrupt(iname.as_str().unwrap(), val.as_hash().unwrap())
                    }
                }
            }
            // Don't do any further processing on derived peripherals
            return;
        }

        // Handle deletions
        if let Some(deletions) = pmod.get(&"_delete".to_yaml()) {
            match deletions {
                Yaml::String(rspec) => {
                    self.delete_register(rspec);
                }
                Yaml::Array(deletions) => {
                    for rspec in deletions {
                        self.delete_register(rspec.as_str().unwrap());
                    }
                }
                Yaml::Hash(deletions) => {
                    for (rspec, val) in deletions {
                        match rspec.as_str().unwrap() {
                            "_registers" => {
                                for rspec in val.as_vec().unwrap() {
                                    self.delete_register(rspec.as_str().unwrap());
                                }
                            }
                            "_interrupts" => {
                                for ispec in val.as_vec().unwrap() {
                                    self.delete_interrupt(ispec.as_str().unwrap());
                                }
                            }
                            rspec => self.delete_register(rspec),
                        }
                    }
                }
                _ => {}
            }
        }

        // Handle modifications
        for (rspec, rmod) in pmod.get_hash("_modify").unwrap_or(&Hash::new()) {
            let rmod = rmod.as_hash().unwrap();
            match rspec.as_str().unwrap() {
                "_registers" => {
                    for (rspec, val) in rmod {
                        self.modify_register(rspec.as_str().unwrap(), val.as_hash().unwrap());
                    }
                }
                "_interrupts" => {
                    for (ispec, val) in rmod {
                        self.modify_interrupt(ispec.as_str().unwrap(), val.as_hash().unwrap());
                    }
                }
                "_cluster" => {
                    for (cspec, val) in rmod {
                        self.modify_cluster(cspec.as_str().unwrap(), val.as_hash().unwrap());
                    }
                }
                rspec => self.modify_register(rspec, rmod),
            }
        }

        // Handle strips
        match pmod.get(&"_strip".to_yaml()) {
            Some(Yaml::String(prefix)) => self.strip(prefix, false),
            Some(Yaml::Array(array)) => {
                for prefix in array {
                    self.strip(prefix.as_str().unwrap(), false);
                }
            }
            _ => {}
        }
        match pmod.get(&"_strip_end".to_yaml()) {
            Some(Yaml::String(suffix)) => self.strip(suffix, true),
            Some(Yaml::Array(array)) => {
                for suffix in array {
                    self.strip(suffix.as_str().unwrap(), true);
                }
            }
            _ => {}
        }

        // Handle additions
        for (rname, radd) in pmod.get_hash("_add").unwrap_or(&Hash::new()) {
            let radd = radd.as_hash().unwrap();
            match rname.as_str().unwrap() {
                "_registers" => {
                    for (rname, val) in radd {
                        self.add_register(rname.as_str().unwrap(), val.as_hash().unwrap())
                    }
                }
                "_interrupts" => {
                    for (iname, val) in radd {
                        self.add_interrupt(iname.as_str().unwrap(), val.as_hash().unwrap())
                    }
                }
                rname => self.add_register(rname, radd),
            }
        }

        for (rname, rderive) in pmod.get_hash("_derive").unwrap_or(&Hash::new()) {
            let rderive = rderive.as_hash().unwrap();
            let rname = rname.as_str().unwrap();
            match rname {
                "_registers" => {
                    for (rname, val) in rderive {
                        self.copy_register(rname.as_str().unwrap(), val.as_hash().unwrap());
                    }
                }
                "_interrupts" => panic!("deriving interrupts not implemented yet: {}", rname),
                _ => self.copy_register(rname, rderive),
            }
        }

        // Handle registers
        for (rspec, register) in pmod {
            let rspec = rspec.as_str().unwrap();
            if !rspec.starts_with("_") {
                self.process_register(rspec, register.as_hash().unwrap(), update_fields)
            }
        }

        // Handle register arrays
        for (rspec, rmod) in pmod.get_hash("_array").unwrap_or(&Hash::new()) {
            self.collect_in_array(rspec.as_str().unwrap(), rmod.as_hash().unwrap());
        }

        // Handle clusters
        for (cname, cmod) in pmod.get_hash("_cluster").unwrap_or(&Hash::new()) {
            self.collect_in_cluster(cname.as_str().unwrap(), cmod.as_hash().unwrap());
        }
    }

    fn iter_clusters<'a, 'b>(&mut self, _spec: &str) -> ClusterMatchIterMut<'a, 'b> {
        todo!()
    }

    fn add_interrupt(&mut self, iname: &str, iadd: &Hash) {
        assert!(
            self.interrupt.iter().find(|i| &i.name == iname).is_none(),
            "peripheral {} already has an interrupt {}",
            self.name,
            iname
        );
        self.interrupt.push(make_interrupt(iname, iadd));
    }

    fn modify_interrupt(&mut self, ispec: &str, imod: &Hash) {
        for itag in self.iter_interrupts(ispec) {
            if let Some(name) = imod.get_str("name") {
                itag.name = name.into();
            }
            if let Some(description) = imod.get_str("description") {
                itag.description = if description.is_empty() {
                    None
                } else {
                    Some(description.into())
                };
            }
            if let Some(value) = imod.get_i64("value") {
                itag.value = value as u32;
            }
        }
    }

    fn delete_interrupt(&mut self, ispec: &str) {
        self.interrupt.retain(|i| !(matchname(&i.name, ispec)));
    }

    fn modify_register(&mut self, rspec: &str, rmod: &Hash) {
        for rtag in self.iter_registers(rspec) {
            rtag.modify_from(make_register(rmod), VAL_LVL).unwrap();
        }
    }

    fn add_register(&mut self, rname: &str, radd: &Hash) {
        if self.reg_iter().find(|r| r.name == rname).is_some() {
            panic!("peripheral {} already has a register {}", self.name, rname);
        }
        self.registers
            .get_or_insert_with(Default::default)
            .push(RegisterCluster::Register(Register::Single(
                make_register(radd)
                    .name(rname.into())
                    .build(VAL_LVL)
                    .unwrap(),
            )));
    }

    fn copy_register(&mut self, rname: &str, rderive: &Hash) {
        let srcname = rderive.get_str("_from").unwrap_or_else(|| {
            panic!(
                "derive: source register not given, please add a _from field to {}",
                rname
            )
        });

        let mut source = self
            .reg_iter()
            .find(|p| &p.name == srcname)
            .unwrap_or_else(|| {
                panic!(
                    "peripheral {} does not have register {}",
                    self.name, srcname
                )
            })
            .clone();
        let fixes = make_register(rderive)
            .name(rname.into())
            .display_name(Some(rname.into()));
        // Modifying fields in derived register not implemented
        source.modify_from(fixes, VAL_LVL).unwrap();
        if let Some(ptag) = self.reg_iter_mut().find(|r| &r.name == rname) {
            source.address_offset = ptag.address_offset;
            *ptag = source;
        } else {
            self.registers
                .as_mut()
                .unwrap()
                .push(RegisterCluster::Register(source))
        }
    }

    fn delete_register(&mut self, rspec: &str) {
        // TODO: delete registers in clusters
        if let Some(registers) = &mut self.registers {
            registers.retain(
                |r| !matches!(r, RegisterCluster::Register(r) if matchname(&r.name, rspec)),
            );
        }
    }

    fn modify_cluster(&mut self, cspec: &str, cmod: &Hash) {
        for ctag in self.iter_clusters(cspec) {
            ctag.modify_from(make_cluster(cmod), VAL_LVL).unwrap();
        }
    }

    fn strip(&mut self, substr: &str, strip_end: bool) {
        let regex = create_regex_from_pattern(substr, strip_end);
        for rtag in self.reg_iter_mut() {
            rtag.name = regex.replace(&rtag.name, "").to_string();

            if let Some(dname) = rtag.display_name.as_mut() {
                *dname = regex.replace(&dname, "").to_string();
            }
        }
    }

    fn collect_in_array(&mut self, rspec: &str, rmod: &Hash) {
        if let Some(regs) = self.registers.as_mut() {
            let mut registers = Vec::new();
            let mut place = usize::MAX;
            let mut i = 0;
            let (li, ri) = spec_ind(rspec);
            while i < regs.len() {
                match &regs[i] {
                    RegisterCluster::Register(Register::Single(r)) if matchname(&r.name, rspec) => {
                        if let RegisterCluster::Register(Register::Single(r)) = regs.remove(i) {
                            registers.push(r);
                            place = place.min(i);
                        }
                    }
                    _ => i += 1,
                }
            }
            if registers.is_empty() {
                panic!("{}: registers {} not found", self.name, rspec);
            }
            registers.sort_by(|r1, r2| r1.address_offset.cmp(&r2.address_offset));
            let dim = registers.len();
            let dim_index = if rmod.contains_key(&"_start_from_zero".to_yaml()) {
                (0..dim).map(|v| v.to_string()).collect::<Vec<_>>()
            } else {
                registers
                    .iter()
                    .map(|r| r.name[li..r.name.len() - ri].to_string())
                    .collect::<Vec<_>>()
            };
            let offsets = registers
                .iter()
                .map(|r| r.address_offset)
                .collect::<Vec<_>>();
            let bitmasks = registers
                .iter()
                .map(RegisterInfo::get_bitmask)
                .collect::<Vec<_>>();
            let dim_increment = if dim > 1 { offsets[1] - offsets[0] } else { 0 };
            if !(check_offsets(&offsets, dim_increment)
                && bitmasks.iter().all(|&m| m == bitmasks[0]))
            {
                panic!(
                    "{}: registers cannot be collected into {} array",
                    self.name, rspec
                );
            }
            let mut rinfo = registers.swap_remove(0);
            if let Some(name) = rmod.get_str("name") {
                rinfo.name = name.into();
            } else {
                rinfo.name = format!("{}%s{}", &rspec[..li], &rspec[rspec.len() - ri..]);
            }
            if dim_index[0] == "0" {
                if let Some(desc) = rinfo.description.as_mut() {
                    *desc = desc.replace('0', "%s");
                }
            }
            let mut reg = Register::Array(
                rinfo,
                DimElement::builder()
                    .dim(dim as u32)
                    .dim_increment(dim_increment)
                    .dim_index(Some(dim_index))
                    .build(VAL_LVL)
                    .unwrap(),
            );
            reg.process(rmod, &self.name, true);
            regs.insert(place, RegisterCluster::Register(reg));
        }
    }

    fn collect_in_cluster(&mut self, cname: &str, cmod: &Hash) {
        if let Some(regs) = self.registers.as_mut() {
            let mut rdict = linked_hash_map::LinkedHashMap::new();
            let mut first = true;
            let mut check = true;
            let mut dim = 0;
            let mut dim_index = Vec::new();
            let mut dim_increment = 0;
            let mut offsets = Vec::new();
            let mut place = usize::MAX;
            let mut rspecs = Vec::new();

            for rspec in cmod.keys() {
                let rspec = rspec.as_str().unwrap();
                if rspec == "description" {
                    continue;
                }
                rspecs.push(rspec.to_string());
                let mut registers = Vec::new();
                let mut i = 0;
                while i < regs.len() {
                    match &regs[i] {
                        RegisterCluster::Register(Register::Single(r))
                            if matchname(&r.name, rspec) =>
                        {
                            if let RegisterCluster::Register(Register::Single(r)) = regs.remove(i) {
                                registers.push(r);
                                place = place.min(i);
                            }
                        }
                        _ => i += 1,
                    }
                }
                if registers.is_empty() {
                    panic!("{}: registers {} not found", self.name, rspec);
                }
                registers.sort_by(|r1, r2| r1.address_offset.cmp(&r2.address_offset));
                let bitmasks = registers
                    .iter()
                    .map(RegisterInfo::get_bitmask)
                    .collect::<Vec<_>>();
                let new_dim_index = registers
                    .iter()
                    .map(|r| {
                        let match_rspec = matchsubspec(&r.name, rspec).unwrap();
                        let (li, ri) = spec_ind(match_rspec);
                        r.name[li..r.name.len() - ri].to_string()
                    })
                    .collect::<Vec<_>>();
                if first {
                    dim = registers.len();
                    dim_index = new_dim_index;
                    dim_increment = 0;
                    offsets = registers
                        .iter()
                        .map(|r| r.address_offset)
                        .collect::<Vec<_>>();
                    if dim > 1 {
                        dim_increment = offsets[1] - offsets[0];
                    }
                    if !(check_offsets(&offsets, dim_increment)
                        && bitmasks.iter().all(|&m| m == bitmasks[0]))
                    {
                        check = false;
                        break;
                    }
                } else {
                    if (dim != registers.len())
                        || (dim_index != new_dim_index)
                        || (!check_offsets(&offsets, dim_increment))
                        || (!bitmasks.iter().all(|&m| m == bitmasks[0]))
                    {
                        check = false;
                        break;
                    }
                }
                rdict.insert(rspec.to_string(), registers);
                first = false;
            }
            if !check {
                panic!(
                    "{}: registers cannot be collected into {} cluster",
                    self.name, cname
                );
            }
            let address_offset = rdict
                .values()
                .min_by(|rs1, rs2| rs1[0].address_offset.cmp(&rs2[0].address_offset))
                .unwrap()[0]
                .address_offset;
            let mut children = Vec::new();
            for (rspec, mut registers) in rdict.into_iter() {
                let mut reg = Register::Single(registers.swap_remove(0));
                let rmod = cmod.get_hash(rspec.as_str()).unwrap();
                reg.process(rmod, &self.name, true);
                if let Some(name) = rmod.get_str("name") {
                    reg.name = name.into();
                } else {
                    let (li, ri) = spec_ind(&rspec);
                    reg.name = format!("{}{}", &rspec[..li], &rspec[rspec.len() - ri..]);
                }
                reg.address_offset = reg.address_offset - address_offset;
                children.push(RegisterCluster::Register(reg));
            }
            let cinfo = ClusterInfo::builder()
                .name(cname.into())
                .description(Some(if let Some(desc) = cmod.get_str("description") {
                    desc.into()
                } else {
                    format!("Cluster {}, containing {}", cname, rspecs.join(", "))
                }))
                .address_offset(address_offset)
                .children(children)
                .build(VAL_LVL)
                .unwrap();
            let cluster = Cluster::Array(
                cinfo,
                DimElement::builder()
                    .dim(dim as u32)
                    .dim_increment(dim_increment)
                    .dim_index(Some(dim_index))
                    .build(VAL_LVL)
                    .unwrap(),
            );
            regs.insert(place, RegisterCluster::Cluster(cluster));
        }
    }

    fn process_register(&mut self, rspec: &str, rmod: &Hash, update_fields: bool) {
        // Find all registers that match the spec
        let mut rcount = 0;
        let pname = self.name.clone();
        for rtag in self.iter_registers(rspec) {
            rcount += 1;
            rtag.process(rmod, &pname, update_fields);
        }
        if rcount == 0 {
            panic!("Could not find {}:{}", &pname, rspec);
        }
    }
}

type FieldIterMut<'a> = OptIterMut<'a, Field, std::slice::IterMut<'a, Field>>;
type FieldMatchIterMut<'a, 'b> = MatchIterMut<'a, 'b, Field, FieldIterMut<'a>>;

/// Collecting methods for processing register contents
trait RegisterExt {
    /// Work through a register, handling all fields
    fn process(&mut self, rmod: &Hash, pname: &str, update_fields: bool);

    /// Add fname given by fadd to rtag
    fn add_field(&mut self, fname: &str, fadd: &Hash);

    /// Delete fields matched by fspec inside rtag
    fn delete_field(&mut self, fspec: &str);

    /// Clear contents of fields matched by fspec inside rtag
    fn clear_field(&mut self, fspec: &str);

    /// Iterates over all fields
    fn iter_all_fields<'a>(&'a mut self) -> FieldIterMut<'a>;

    /// Iterates over all fields that match fspec and live inside rtag
    fn iter_fields<'a, 'b>(&'a mut self, spec: &'b str) -> FieldMatchIterMut<'a, 'b>;

    /// Work through a field, handling either an enum or a range
    fn process_field(&mut self, pname: &str, fspec: &str, fmod: &Yaml);

    /// Add an enumeratedValues given by field to all fspec in rtag
    fn process_field_enum(&mut self, pname: &str, fspec: &str, fmod: &Hash, usage: Usage);

    /// Add a writeConstraint range given by field to all fspec in rtag
    fn process_field_range(&mut self, pname: &str, fspec: &str, fmod: &Vec<Yaml>);

    /// Delete substring from bitfield names inside rtag. Strips from the
    /// beginning of the name by default.
    fn strip(&mut self, substr: &str, strip_end: bool);

    /// Modify fspec inside rtag according to fmod
    fn modify_field(&mut self, fspec: &str, fmod: &Hash);

    /// Merge all fspec in rtag.
    /// Support list of field to auto-merge, and dict with fspec or list of fspec
    fn merge_fields(&mut self, key: &str, value: Option<&Yaml>);

    /// Split all fspec in rtag.
    /// Name and description can be customized with %s as a placeholder to the iterator value
    fn split_fields(&mut self, fspec: &str, fsplit: &Hash);

    /// Collect same fields in peripheral into register array
    fn collect_fields_in_array(&mut self, fspec: &str, fmod: &Hash);
}

trait RegisterInfoExt {
    /// Calculate filling of register
    fn get_bitmask(&self) -> u64;
}

impl RegisterInfoExt for RegisterInfo {
    fn get_bitmask(&self) -> u64 {
        let mut mask = 0x0;
        if let Some(fields) = self.fields.as_ref() {
            for ftag in fields {
                mask |= (!0 >> (64 - ftag.bit_range.width)) << ftag.bit_range.offset;
            }
        }
        mask
    }
}

impl RegisterExt for Register {
    fn process(&mut self, rmod: &Hash, pname: &str, update_fields: bool) {
        // Handle deletions
        match rmod.get(&"_delete".to_yaml()) {
            Some(Yaml::String(fspec)) => self.delete_field(fspec),
            Some(Yaml::Array(array)) => {
                for fspec in array {
                    self.delete_field(fspec.as_str().unwrap());
                }
            }
            _ => {}
        }

        // Handle field clearing
        for fspec in rmod.get_vec("_clear").unwrap_or(&Vec::new()) {
            self.clear_field(fspec.as_str().unwrap());
        }

        // Handle modifications
        if let Some(from) = rmod.get_hash("_modify") {
            for (fspec, fmod) in from {
                if let Yaml::Hash(fmod) = fmod {
                    self.modify_field(fspec.as_str().unwrap(), fmod);
                }
            }
            self.modify_from(make_register(&from), VAL_LVL).ok();
        }
        // Handle additions
        for (fname, fadd) in rmod.get_hash("_add").unwrap_or(&Hash::new()) {
            self.add_field(fname.as_str().unwrap(), fadd.as_hash().unwrap());
        }

        // Handle merges
        match rmod.get(&"_merge".to_yaml()) {
            Some(Yaml::Hash(h)) => {
                for (fspec, fmerge) in h {
                    self.merge_fields(fspec.as_str().unwrap(), Some(fmerge));
                }
            }
            Some(Yaml::Array(a)) => {
                for fspec in a {
                    self.merge_fields(fspec.as_str().unwrap(), None);
                }
            }
            _ => {}
        }

        // Handle splits
        match rmod.get(&"_split".to_yaml()) {
            Some(Yaml::Hash(h)) => {
                for (fspec, fsplit) in h {
                    self.split_fields(fspec.as_str().unwrap(), fsplit.as_hash().unwrap());
                }
            }
            Some(Yaml::Array(a)) => {
                for fspec in a {
                    self.split_fields(fspec.as_str().unwrap(), &Hash::new());
                }
            }
            _ => {}
        }

        // Handle strips
        match rmod.get(&"_strip".to_yaml()) {
            Some(Yaml::String(prefix)) => self.strip(prefix, false),
            Some(Yaml::Array(array)) => {
                for prefix in array {
                    self.strip(prefix.as_str().unwrap(), false);
                }
            }
            _ => {}
        }
        match rmod.get(&"_strip_end".to_yaml()) {
            Some(Yaml::String(suffix)) => self.strip(suffix, true),
            Some(Yaml::Array(array)) => {
                for suffix in array {
                    self.strip(suffix.as_str().unwrap(), true);
                }
            }
            _ => {}
        }

        // Handle fields
        if update_fields {
            for (fspec, field) in rmod {
                let fspec = fspec.as_str().unwrap();
                if !fspec.starts_with("_") {
                    self.process_field(pname, fspec, field)
                }
            }
        }

        // Handle field arrays
        for (fspec, fmod) in rmod.get_hash("_array").unwrap_or(&Hash::new()) {
            self.collect_fields_in_array(fspec.as_str().unwrap(), fmod.as_hash().unwrap())
        }
    }

    fn iter_all_fields<'a>(&'a mut self) -> FieldIterMut<'a> {
        OptIterMut(self.fields.as_mut().map(|f| f.iter_mut()))
    }

    fn iter_fields<'a, 'b>(&'a mut self, spec: &'b str) -> FieldMatchIterMut<'a, 'b> {
        self.iter_all_fields().matched(spec)
    }

    fn strip(&mut self, substr: &str, strip_end: bool) {
        let regex = create_regex_from_pattern(substr, strip_end);
        if let Some(fields) = self.fields.as_mut() {
            for ftag in fields {
                ftag.name = regex.replace(&ftag.name, "").to_string();
            }
        }
    }

    fn modify_field(&mut self, fspec: &str, fmod: &Hash) {
        for ftag in self.iter_fields(fspec) {
            if let Some(wc) = fmod
                .get(&"_write_constraint".to_yaml())
                .or_else(|| fmod.get(&"writeConstraint".to_yaml()))
                .map(|value| match value {
                    Yaml::String(s) if s == "none" => {
                        // Completely remove the existing writeConstraint
                        None
                    }
                    Yaml::String(s) if s == "enum" => {
                        // Only allow enumerated values
                        Some(WriteConstraint::UseEnumeratedValues(true))
                    }
                    Yaml::Array(a) => {
                        // Allow a certain range
                        Some(WriteConstraint::Range(WriteConstraintRange {
                            min: parse_i64(&a[0]).unwrap() as u64,
                            max: parse_i64(&a[1]).unwrap() as u64,
                        }))
                    }
                    _ => panic!("Unknown writeConstraint type {:?}", value),
                })
            {
                ftag.write_constraint = wc;
            }
            // For all other tags, just set the value
            ftag.modify_from(make_field(fmod), VAL_LVL).unwrap();
        }
    }

    fn add_field(&mut self, fname: &str, fadd: &Hash) {
        if self.iter_all_fields().find(|f| f.name == fname).is_some() {
            panic!("register {} already has a field {}", self.name, fname);
        }
        // TODO: add field arrays
        let fnew = Field::Single(make_field(fadd).name(fname.into()).build(VAL_LVL).unwrap());
        self.fields.get_or_insert_with(Default::default).push(fnew);
    }

    fn delete_field(&mut self, fspec: &str) {
        if let Some(fields) = self.fields.as_mut() {
            fields.retain(|f| !(matchname(&f.name, fspec)));
        }
    }

    fn clear_field(&mut self, fspec: &str) {
        for ftag in self.iter_fields(fspec) {
            ftag.enumerated_values = Vec::new();
            ftag.write_constraint = None;
        }
    }

    fn merge_fields(&mut self, key: &str, value: Option<&Yaml>) {
        let (name, names) = match value {
            Some(Yaml::String(value)) => (
                key.to_string(),
                self.iter_fields(&value)
                    .map(|f| f.name.to_string())
                    .collect(),
            ),
            Some(Yaml::Array(value)) => {
                let mut names = Vec::new();
                for fspec in value {
                    names.extend(
                        self.iter_fields(fspec.as_str().unwrap())
                            .map(|f| f.name.to_string()),
                    );
                }
                (key.to_string(), names)
            }
            Some(_) => panic!("Invalid usage of merge for {}.{}", self.name, key),
            None => {
                let names: Vec<String> =
                    self.iter_fields(key).map(|f| f.name.to_string()).collect();
                let name = commands::util::longest_common_prefix(
                    names.iter().map(|n| n.as_str()).collect(),
                )
                .to_string();
                (name, names)
            }
        };

        if names.is_empty() {
            panic!("Could not find any fields to merge {}.{}", self.name, key);
        }
        let mut bitwidth = 0;
        let mut bitoffset = u32::MAX;
        let mut first = true;
        let mut desc = None;
        if let Some(fields) = self.fields.as_mut() {
            for f in fields.iter_mut() {
                if names.contains(&f.name) {
                    if first {
                        desc = f.description.clone();
                        first = false;
                    }
                    bitwidth += f.bit_range.width;
                    bitoffset = bitoffset.min(f.bit_range.offset);
                }
            }
            fields.retain(|f| !names.contains(&f.name));
            fields.push(Field::Single(
                FieldInfo::builder()
                    .name(name)
                    .description(desc)
                    .bit_range(BitRange::from_offset_width(bitoffset, bitwidth))
                    .build(VAL_LVL)
                    .unwrap(),
            ));
        }
    }

    fn collect_fields_in_array(&mut self, fspec: &str, fmod: &Hash) {
        if let Some(fs) = self.fields.as_mut() {
            let mut fields = Vec::new();
            let mut place = usize::MAX;
            let mut i = 0;
            let (li, ri) = spec_ind(fspec);
            while i < fs.len() {
                match &fs[i] {
                    Field::Single(f) if matchname(&f.name, fspec) => {
                        if let Field::Single(f) = fs.remove(i) {
                            fields.push(f);
                            place = place.min(i);
                        }
                    }
                    _ => i += 1,
                }
            }
            if fields.is_empty() {
                panic!("{}: fields {} not found", self.name, fspec);
            }
            fields.sort_by(|f1, f2| f1.bit_range.offset.cmp(&f2.bit_range.offset));
            let dim = fields.len();
            let dim_index = if fmod.contains_key(&"_start_from_zero".to_yaml()) {
                (0..dim).map(|v| v.to_string()).collect::<Vec<_>>()
            } else {
                fields
                    .iter()
                    .map(|f| f.name[li..f.name.len() - ri].to_string())
                    .collect::<Vec<_>>()
            };
            let offsets = fields
                .iter()
                .map(|f| f.bit_range.offset)
                .collect::<Vec<_>>();
            let dim_increment = if dim > 1 { offsets[1] - offsets[0] } else { 0 };
            if !check_offsets(&offsets, dim_increment) {
                panic!(
                    "{}: registers cannot be collected into {} array",
                    self.name, fspec
                );
            }
            let mut finfo = fields.swap_remove(0);
            if let Some(name) = fmod.get_str("name") {
                finfo.name = name.into();
            } else {
                finfo.name = format!("{}%s{}", &fspec[..li], &fspec[fspec.len() - ri..]);
            }
            if dim_index[0] == "0" {
                if let Some(desc) = finfo.description.as_mut() {
                    *desc = desc.replace('0', "%s");
                }
            }
            let field = Field::Array(
                finfo,
                DimElement::builder()
                    .dim(dim as u32)
                    .dim_increment(dim_increment)
                    .dim_index(Some(dim_index))
                    .build(VAL_LVL)
                    .unwrap(),
            );
            //field.process(fmod, &self.name, true);
            fs.insert(place, field);
        }
    }
    fn split_fields(&mut self, fspec: &str, fsplit: &Hash) {
        let mut it = self.iter_fields(fspec);
        let (new_fields, name) = match (it.next(), it.next()) {
            (None, _) => panic!("Could not find any fields to split {}.{}", self.name, fspec),
            (Some(_), Some(_)) => panic!(
                "Only one field can be spitted at time {}.{}",
                self.name, fspec
            ),
            (Some(first), None) => {
                let name = if let Some(n) = fsplit.get_str("name") {
                    n.to_string()
                } else {
                    first.name.clone() + "%s"
                };
                let desc = if let Some(d) = fsplit.get_str("description") {
                    Some(d.to_string())
                } else {
                    first.description.clone()
                };
                let bitoffset = first.bit_range.offset;
                (
                    (0..first.bit_range.width)
                        .map(|i| {
                            let is = i.to_string();
                            Field::Single(
                                FieldInfo::builder()
                                    .name(name.replace("%s", &is))
                                    .description(desc.clone().map(|d| d.replace("%s", &is)))
                                    .bit_range(BitRange::from_offset_width(bitoffset + i, 1))
                                    .build(VAL_LVL)
                                    .unwrap(),
                            )
                        })
                        .collect::<Vec<_>>(),
                    first.name.to_string(),
                )
            }
        };
        if let Some(fields) = self.fields.as_mut() {
            fields.retain(|f| f.name != name);
            fields.extend(new_fields);
        }
    }

    fn process_field(&mut self, pname: &str, fspec: &str, fmod: &Yaml) {
        match fmod {
            Yaml::Hash(fmod) => {
                let is_read = fmod.contains_key(&"_read".to_yaml());
                let is_write = fmod.contains_key(&"_write".to_yaml());
                if !(is_read || is_write) {
                    self.process_field_enum(pname, fspec, fmod, Usage::ReadWrite);
                } else {
                    if is_read {
                        self.process_field_enum(
                            pname,
                            fspec,
                            fmod.get_hash("_read").unwrap(),
                            Usage::Read,
                        );
                    }
                    if is_write {
                        self.process_field_enum(
                            pname,
                            fspec,
                            fmod.get_hash("_write").unwrap(),
                            Usage::Write,
                        );
                    }
                }
            }
            Yaml::Array(fmod) if fmod.len() == 2 => {
                self.process_field_range(pname, fspec, fmod);
            }
            _ => {}
        }
    }

    fn process_field_enum(&mut self, pname: &str, fspec: &str, mut fmod: &Hash, usage: Usage) {
        let mut replace_if_exists = false;
        if let Some(h) = fmod.get_hash("_replace_enum") {
            fmod = h;
            replace_if_exists = true;
        }

        let mut name = String::new();
        let mut derived = None;
        let mut en = None;
        for ftag in self.iter_fields(fspec) {
            // sorted_fields(list(self.iter_fields(fspec)))
            if let Some(d) = fmod.get_str("_derivedFrom") {
                derived = Some(d.to_string());
            } else {
                name = ftag.name.to_string();
            }

            if let Some(derived) = derived.as_ref() {
                ftag.enumerated_values
                    .push(make_derived_enumerated_values(derived));
            } else {
                if en.is_none() {
                    en = Some(make_enumerated_values(&name, fmod, usage));
                }

                for ev in &ftag.enumerated_values {
                    let ev_usage = if !ev.values.is_empty() {
                        ev.usage()
                    } else {
                        // This is a derived enumeratedValues => Try to find the
                        // original definition to extract its <usage>
                        /*let derived_name = ev.derived_from.clone();
                        assert!(derived_name.is_some());
                        let mut derived_enums = self
                            .fields
                            .as_ref()
                            .unwrap()
                            .iter()
                            .flat_map(|f| f.enumerated_values.iter())
                            .filter(|e| e.name == derived_name);
                        match (derived_enums.next(), derived_enums.next()) {
                            (Some(e), None) => e.usage(),
                            (None, _) => panic!("{}: field {} derives enumeratedValues {} which could not be found", pname, &name, derived_name.unwrap()),
                            (Some(_), Some(_)) => panic!("{}: field {} derives enumeratedValues {} which was found multiple times", pname, &name, derived_name.unwrap()),
                        }*/
                        usage
                    };

                    if ev_usage == en.as_ref().unwrap().usage() || ev_usage == Usage::ReadWrite {
                        if replace_if_exists {
                            //                            ftag.remove(ev)
                        } else {
                            panic!(
                                "{}: field {} already has enumeratedValues for {:?}",
                                pname, name, ev_usage
                            );
                        }
                    }
                }

                if let Some(en) = en.clone() {
                    derived = en.name.clone();
                    ftag.enumerated_values.push(en);
                }
            }
        }
        if derived.is_none() {
            panic!("Could not find {}:{}.{}", pname, &self.name, fspec);
        }
    }

    fn process_field_range(&mut self, pname: &str, fspec: &str, fmod: &Vec<Yaml>) {
        let mut set_any = false;
        for ftag in self.iter_fields(fspec) {
            ftag.write_constraint = Some(WriteConstraint::Range(WriteConstraintRange {
                min: parse_i64(&fmod[0]).unwrap() as u64,
                max: parse_i64(&fmod[1]).unwrap() as u64,
            }));
            set_any = true;
        }
        if !set_any {
            panic!("Could not find {}:{}.{}", pname, &self.name, fspec);
        }
    }
}

/// Create regex from pattern to match start or end of string
fn create_regex_from_pattern(substr: &str, strip_end: bool) -> Regex {
    // TODO: optimize
    // make matching non-greedy
    let mut regex = Glob::new(substr)
        .unwrap()
        .regex()
        .replace('*', "*?")
        .replace('.', r"[\w\d]");
    // change to start of string search
    if !strip_end {
        regex = "^".to_string() + &Regex::new(r"\$$").unwrap().replace(&regex, "");
    }
    Regex::new(&regex).unwrap()
}

/// Given a name and a dict of values which maps variant names to (value,
/// description), returns an enumeratedValues Element.
fn make_enumerated_values(name: &str, values: &Hash, usage: Usage) -> EnumeratedValues {
    if name.as_bytes()[0].is_ascii_digit() {
        panic!("enumeratedValue {}: can't start with a number", name);
    }
    let ev = EnumeratedValues::builder()
        .name(Some(
            name.to_string()
                + match usage {
                    Usage::Read => "R",
                    Usage::Write => "W",
                    Usage::ReadWrite => "",
                },
        ))
        .usage(Some(usage));
    let mut h = std::collections::BTreeMap::new();
    for (n, vd) in values {
        let vname = n.as_str().unwrap();
        if !vname.starts_with("_") {
            if vname.as_bytes()[0].is_ascii_digit() {
                panic!(
                    "enumeratedValue {}.{}: can't start with a number",
                    name, vname
                );
            }
            let vd = vd.as_vec().unwrap();
            let value = parse_i64(&vd[0]).unwrap() as u64;
            let description = vd.get(1).and_then(Yaml::as_str).unwrap_or_else(|| {
                panic!(
                    "enumeratedValue {}: can't have empty description for value {}",
                    name, value
                )
            });
            use std::collections::btree_map::Entry;
            match h.entry(value) {
                Entry::Occupied(_) => {
                    panic!("enumeratedValue {}: can't have duplicate values", name)
                }
                Entry::Vacant(e) => {
                    e.insert((vname.to_string(), description.to_string()));
                }
            }
        }
    }
    ev.values(
        h.into_iter()
            .map(|(value, vd)| {
                EnumeratedValue::builder()
                    .name(vd.0)
                    .value(Some(value))
                    .description(Some(vd.1))
                    .build(VAL_LVL)
                    .unwrap()
            })
            .collect(),
    )
    .build(VAL_LVL)
    .unwrap()
}

/// Returns an enumeratedValues Element which is derivedFrom name
fn make_derived_enumerated_values(name: &str) -> EnumeratedValues {
    EnumeratedValues::builder()
        .derived_from(Some(name.into()))
        .build(VAL_LVL)
        .unwrap()
}

fn make_address_blocks(value: &Vec<Yaml>) -> Vec<AddressBlock> {
    value
        .iter()
        .map(|h| make_address_block(h.as_hash().unwrap()))
        .collect::<Vec<_>>()
}
fn make_address_block(h: &Hash) -> AddressBlock {
    AddressBlock {
        offset: h.get_i64("offset").unwrap() as u32,
        size: h.get_i64("size").unwrap() as u32,
        usage: h
            .get_str("usage")
            .and_then(AddressBlockUsage::from_str)
            .unwrap(),
    }
}

fn make_field(fadd: &Hash) -> FieldInfoBuilder {
    let mut fnew = FieldInfo::builder()
        .description(fadd.get_str("description").map(String::from))
        .access(fadd.get_str("access").and_then(Access::from_str));

    if let Some(name) = fadd.get_str("name") {
        fnew = fnew.name(name.into());
    }
    if let Some(offset) = fadd.get_i64("bitOffset") {
        fnew = fnew.bit_offset(offset as u32)
    }
    if let Some(width) = fadd.get_i64("bitWidth") {
        fnew = fnew.bit_width(width as u32)
    }

    fnew
}

fn make_register(radd: &Hash) -> RegisterInfoBuilder {
    let mut rnew = RegisterInfo::builder()
        .display_name(radd.get_str("displayName").map(String::from))
        .description(radd.get_str("description").map(String::from))
        .alternate_group(radd.get_str("alternateGroup").map(String::from))
        .alternate_register(radd.get_str("alternateRegister").map(String::from))
        .properties(get_register_properties(radd))
        .fields(radd.get_hash("fields").map(|h| {
            h.iter()
                .map(|(fname, val)| {
                    Field::Single(
                        make_field(val.as_hash().unwrap())
                            .name(fname.as_str().unwrap().into())
                            .build(VAL_LVL)
                            .unwrap(),
                    )
                })
                .collect()
        }));

    if let Some(name) = radd.get_str("name") {
        rnew = rnew.name(name.into());
    }
    if let Some(address_offset) = radd.get_i64("addressOffset") {
        rnew = rnew.address_offset(address_offset as u32);
    }
    rnew
}

fn make_cluster(cadd: &Hash) -> ClusterInfoBuilder {
    let mut cnew = ClusterInfo::builder()
        .description(cadd.get_str("description").map(String::from))
        .default_register_properties(get_register_properties(cadd));

    if let Some(name) = cadd.get_str("name") {
        cnew = cnew.name(name.into());
    }
    if let Some(address_offset) = cadd.get_i64("addressOffset") {
        cnew = cnew.address_offset(address_offset as u32);
    }
    cnew
}

fn make_interrupt(iname: &str, iadd: &Hash) -> Interrupt {
    Interrupt {
        name: iname.into(),
        description: iadd.get_str("description").map(String::from),
        value: iadd
            .get_i64("value")
            .unwrap_or_else(|| panic!("value is absent for interrupt {}", iname))
            as u32,
    }
}

fn make_peripheral(padd: &Hash) -> PeripheralBuilder {
    let mut pnew = Peripheral::builder()
        .display_name(padd.get_str("displayName").map(String::from))
        .version(padd.get_str("version").map(String::from))
        .description(padd.get_str("description").map(String::from))
        .group_name(padd.get_str("groupName").map(String::from))
        .interrupt(
            padd.get_hash("interrupts")
                .map(|value| {
                    value
                        .iter()
                        .map(|(iname, val)| {
                            make_interrupt(iname.as_str().unwrap(), val.as_hash().unwrap())
                        })
                        .collect()
                })
                .unwrap_or_else(|| Vec::new()),
        );
    if let Some(name) = padd.get_str("name") {
        pnew = pnew.name(name.into());
    }
    if let Some(base_address) = padd.get_i64("baseAddress") {
        pnew = pnew.base_address(base_address as u64);
    }

    if let Some(derived) = padd.get_str("derivedFrom") {
        pnew.derived_from(Some(derived.into()))
    } else {
        pnew.default_register_properties(get_register_properties(padd))
            .address_block(
                padd.get_hash("addressBlock")
                    .map(|value| vec![make_address_block(value)])
                    .or_else(|| {
                        padd.get_vec("addressBlocks")
                            .map(|value| make_address_blocks(value))
                    }),
            )
            .registers(padd.get_hash("registers").map(|h| {
                h.iter()
                    .map(|(rname, val)| {
                        RegisterCluster::Register(Register::Single(
                            make_register(val.as_hash().unwrap())
                                .name(rname.as_str().unwrap().into())
                                .build(VAL_LVL)
                                .unwrap(),
                        ))
                    })
                    .collect()
            }))
    }
}

/// Find left and right indices of enumeration token in specification string
fn spec_ind(spec: &str) -> (usize, usize) {
    let li = spec
        .bytes()
        .position(|b| b == b'*')
        .or_else(|| spec.bytes().position(|b| b == b'?'))
        .or_else(|| spec.bytes().position(|b| b == b'['))
        .unwrap();
    let ri = spec
        .bytes()
        .rev()
        .position(|b| b == b'*')
        .or_else(|| spec.bytes().rev().position(|b| b == b'?'))
        .or_else(|| spec.bytes().rev().position(|b| b == b']'))
        .unwrap();
    (li, ri)
}

fn check_offsets(offsets: &[u32], dim_increment: u32) -> bool {
    let mut it = offsets.windows(2);
    while let Some(&[o1, o2]) = it.next() {
        if o2 - o1 != dim_increment {
            return false;
        }
    }
    true
}

/*
def natural_keys(text):
    return [int(c) if c.isdigit() else c for c in re.split(r"(\d+)", text)]


def sorted_fields(fields):
    return sorted(fields, key=lambda ftag: natural_keys(ftag.find("name").text))
*/

#[test]
fn stm32f102() {
    process_file(Path::new("/home/burrbull/stm32-rs/devices/stm32f102.yaml")).unwrap();
}
