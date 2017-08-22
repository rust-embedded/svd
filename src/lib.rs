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
