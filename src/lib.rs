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


mod parse;
mod helpers;
use helpers::*;
mod endian;
pub use endian::*;
mod access;
pub use access::*;
mod usage;
pub use usage::*;
mod enumeratedvalue;
pub use enumeratedvalue::*;
mod enumeratedvalues;
pub use enumeratedvalues::*;
mod defaults;
pub use defaults::*;
mod writeconstraintrange;
pub use writeconstraintrange::*;
mod writeconstraint;
pub use writeconstraint::*;
mod bitrange;
pub use bitrange::*;
mod interrupt;
pub use interrupt::*;
mod addressblock;
pub use addressblock::*;
mod field;
pub use field::*;
mod register;
pub use register::*;
mod clusterinfo;
pub use clusterinfo::*;
mod cluster;
pub use cluster::*;
mod registerinfo;
pub use registerinfo::*;
mod registerarrayinfo;
pub use registerarrayinfo::*;
mod registerclusterarrayinfo;
pub use registerclusterarrayinfo::*;
mod registercluster;
mod peripheral;
pub use peripheral::*;
mod cpu;
pub use cpu::*;
mod device;
pub use device::*;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}


/// Parses the contents of a SVD file (XML)
pub fn parse(xml: &str) -> Device {
    let tree = &try!(Element::parse(xml.as_bytes()));
    Device::parse(tree)
}

pub fn encode(device: &Device) -> Element {
    device.encode()
}

trait ElementExt {
    fn get_child_text<K>(&self, k: K) -> Option<String>
    where
        String: PartialEq<K>;
    fn debug(&self);
}

impl ElementExt for Element {
    fn get_child_text<K>(&self, k: K) -> Option<String>
    where
        String: PartialEq<K>,
    {
        self.get_child(k).map(|c| try!(c.text.clone()))
    }

    fn debug(&self) {
        println!("<{}>", self.name);
        for c in &self.children {
            println!("{}: {:?}", c.name, c.text)
        }
        println!("</{}>", self.name);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use bitrange::*;

    use std::fs;
    use std::process::Command;
    use std::fs::File;
    use std::io;
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
