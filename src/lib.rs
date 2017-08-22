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

#![deny(warnings)]


extern crate xmltree;

use xmltree::Element;

mod helpers;
use helpers::*;

mod parse;

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

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

/// Parses the contents of a SVD file (XML)
pub fn parse(xml: &str) -> Device {
    Device::parse(xml)
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

#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    pub cpu: Option<Cpu>,
    pub peripherals: Vec<Peripheral>,
    pub defaults: Defaults,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Device {
    /// Parses a SVD file
    ///
    /// # Panics
    ///
    /// If the input is an invalid SVD file (yay, no error handling)
    pub fn parse(svd: &str) -> Device {
        let tree = &try!(Element::parse(svd.as_bytes()));

        Device {
            name: try!(tree.get_child_text("name")),
            cpu: tree.get_child("cpu").map(Cpu::parse),
            peripherals: try!(tree.get_child("peripherals"))
                .children
                .iter()
                .map(Peripheral::parse)
                .collect(),
            defaults: Defaults::parse(tree),
            _extensible: (),
        }
    }
}


#[derive(Clone, Debug)]
pub struct Cpu {
    pub name: String,
    pub revision: String,
    pub endian: Endian,
    pub mpu_present: bool,
    pub fpu_present: bool,
    pub nvic_priority_bits: u32,
    pub has_vendor_systick: bool,

    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Cpu {
    fn parse(tree: &Element) -> Cpu {
        assert_eq!(tree.name, "cpu");

        Cpu {
            name: try!(tree.get_child_text("name")),
            revision: try!(tree.get_child_text("revision")),
            endian: Endian::parse(try!(tree.get_child("endian"))),
            mpu_present: try!(parse::bool(try!(tree.get_child("mpuPresent")))),
            fpu_present: try!(parse::bool(try!(tree.get_child("fpuPresent")))),
            nvic_priority_bits:
                try!(parse::u32(try!(tree.get_child("nvicPrioBits")))),
            has_vendor_systick:
                try!(parse::bool(try!(tree.get_child("vendorSystickConfig")))),

            _extensible: (),
        }
    }

    pub fn is_cortex_m(&self) -> bool {
        self.name.starts_with("CM")
    }
}

#[derive(Clone, Debug)]
pub struct Peripheral {
    pub name: String,
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub base_address: u32,
    pub interrupt: Vec<Interrupt>,
    /// `None` indicates that the `<registers>` node is not present
    pub registers: Option<Vec<Register>>,
    pub derived_from: Option<String>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Peripheral {
    fn parse(tree: &Element) -> Peripheral {
        assert_eq!(tree.name, "peripheral");

        Peripheral {
            name: try!(tree.get_child_text("name")),
            group_name: tree.get_child_text("groupName"),
            description: tree.get_child_text("description"),
            base_address: try!(parse::u32(try!(tree.get_child("baseAddress")))),
            interrupt: tree.children
                .iter()
                .filter(|t| t.name == "interrupt")
                .map(Interrupt::parse)
                .collect::<Vec<_>>(),
            registers: tree.get_child("registers")
                .map(
                    |rs| {
                        rs.children
                            .iter()
                            .filter_map(Register::parse)
                            .collect()
                    },
                ),
            derived_from: tree.attributes.get("derivedFrom").map(
                |s| {
                    s.to_owned()
                },
            ),
            _extensible: (),
        }
    }
}




