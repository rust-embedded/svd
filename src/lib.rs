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
mod field;
pub use field::*;
mod register;
pub use register::*;
mod registerinfo;
pub use registerinfo::*;
mod registerarrayinfo;
pub use registerarrayinfo::*;
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
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn decode_encode() {
        let path = String::from("./examples");

        let files: Vec<String> = fs::read_dir(&path).unwrap()
        .map(|res| res.unwrap())
        .filter(|f| !f.metadata().unwrap().is_dir())
        .map(|f| f.file_name().into_string().unwrap())
        .filter(|f| !(f.starts_with(".") || f.starts_with("_")))
        .collect();

        println!("Files: {:?}", files);
    
        for name in files {
            let p1 = format!("{}/{}", path, name);

            let mut xml = String::new();
            let mut f = fs::File::open(&p1).unwrap();
            f.read_to_string(&mut xml).unwrap();

            let device = parse(&xml);

            let p2 = format!("{}/{}", String::from("target"), name);
            encode(&device).write(File::create(&p2).unwrap());

            let output1 = Command::new("xmllint").arg("--exc-c14n").arg(p1).output().unwrap();
            let mut f1 = File::create("target/p1.svd").unwrap();
            f1.write_all(&output1.stdout);

            let output2 = Command::new("xmllint").arg("--exc-c14n").arg(p2).output().unwrap();
            let mut f2 = File::create("target/p2.svd").unwrap();
            f2.write_all(&output2.stdout);

            

        }
    }
}