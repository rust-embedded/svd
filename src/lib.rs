//! CMSIS-SVD file parser
//!
//! # Usage
//!
//! ``` no_run
//! extern crate svd_parser as svd;
//!
//! use std::fs::File;
//! use std::io::Read;
//!
//! fn main() {
//!     let xml = &mut String::new();
//!     File::open("STM32F30x.svd").unwrap().read_to_string(xml);
//!
//!     println!("{:?}", svd::parse(xml));
//! }
//! ```
//!
//! # References
//!
//! - [SVD Schema file](https://www.keil.com/pack/doc/CMSIS/SVD/html/schema_1_2_gr.html)
//! - [SVD file database](https://github.com/posborne/cmsis-svd/tree/master/data)
//! - [Sample SVD file](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_Example_pg.html)

//#![deny(warnings)]

extern crate either;
extern crate xmltree;
use xmltree::Element;

#[macro_use]
mod elementext;
mod parse;

mod helpers;
use helpers::*;
mod endian;
pub use endian::Endian;
mod access;
pub use access::Access;
mod usage;
pub use usage::Usage;
mod enumeratedvalue;
pub use enumeratedvalue::EnumeratedValue;
mod enumeratedvalues;
pub use enumeratedvalues::EnumeratedValues;
mod defaults;
pub use defaults::Defaults;
mod writeconstraintrange;
pub use writeconstraintrange::WriteConstraintRange;
mod writeconstraint;
pub use writeconstraint::WriteConstraint;
mod bitrange;
pub use bitrange::BitRange;
mod interrupt;
pub use interrupt::Interrupt;
mod addressblock;
pub use addressblock::AddressBlock;
mod field;
pub use field::Field;
mod register;
pub use register::Register;
mod clusterinfo;
pub use clusterinfo::ClusterInfo;
mod cluster;
pub use cluster::Cluster;
mod registerinfo;
pub use registerinfo::RegisterInfo;
mod registerarrayinfo;
pub use registerarrayinfo::RegisterArrayInfo;
mod registerclusterarrayinfo;
pub use registerclusterarrayinfo::RegisterClusterArrayInfo;
mod registercluster;
mod peripheral;
pub use peripheral::Peripheral;
mod cpu;
pub use cpu::Cpu;
mod device;
pub use device::Device;


/// Parses the contents of a SVD file (XML)
pub fn parse(xml: &str) -> Device {
    let tree = &try_get_child!(Element::parse(xml.as_bytes()));
    Device::parse(tree)
}

pub fn encode(device: &Device) -> Element {
    device.encode()
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::process::Command;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn decode_encode() {
        let path = String::from("./examples");

        let files: Vec<String> = fs::read_dir(&path)
            .unwrap()
            .map(|res| res.unwrap())
            .filter(|all| !all.metadata().unwrap().is_dir())
            .map(|file| file.file_name().into_string().unwrap())
            .filter(|filename| {
                !(filename.starts_with(".") || filename.starts_with("_")) && filename.ends_with(".svd")
            })
            .map(|filename| {
                String::from(Path::new(&filename).file_stem().unwrap().to_str().unwrap())
            })
            .collect();

        println!("Files: {:?}", files);

        for name in files {
            let source_file = format!("{}/{}.svd", path, name);
            let original_file = format!("target/{}-original.svd", name);
            let encoded_file = format!("target/{}-encoded.svd", name);
            let diff_file = format!("target/{}-diff.svd", name);

            // Load orignal target file
            let mut xml = String::new();
            let mut f = fs::File::open(&source_file).unwrap();
            f.read_to_string(&mut xml).unwrap();

            // Parse device info
            let device = parse(&xml);

            // Encode device info

            encode(&device).write(File::create(&encoded_file).unwrap());

            // Normalize source info
            let output = Command::new("xmllint")
                .arg("--c14n")
                .arg(source_file.clone())
                .output()
                .unwrap();
            let mut f = File::create(original_file.clone()).unwrap();
            f.write_all(&output.stdout).unwrap();

            let output = Command::new("xmllint")
                .arg("--format")
                .arg(source_file.clone())
                .output()
                .unwrap();
            let mut f = File::create(original_file.clone()).unwrap();
            f.write_all(&output.stdout).unwrap();

            // Normalise encoded info
            let output = Command::new("xmllint")
                .arg("--c14n")
                .arg(encoded_file.clone())
                .output()
                .unwrap();
            let mut f = File::create(encoded_file.clone()).unwrap();
            f.write_all(&output.stdout).unwrap();

            let output = Command::new("xmllint")
                .arg("--format")
                .arg(encoded_file.clone())
                .output()
                .unwrap();
            let mut f = File::create(encoded_file.clone()).unwrap();
            f.write_all(&output.stdout).unwrap();

            // Diff normalised source and output
            let output = Command::new("diff")
                .arg(original_file)
                .arg(encoded_file)
                .output()
                .unwrap();
            let mut f = File::create(diff_file).unwrap();
            f.write_all(&output.stdout).unwrap();

        }
    }
}
