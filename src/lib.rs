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
//! - [SVD Schema file](https://www.keil.com/pack/doc/CMSIS/SVD/html/group__schema__1__2__gr.html)
//! - [SVD file database](https://github.com/posborne/cmsis-svd/tree/master/data)
//! - [Sample SVD file](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd__example_pg.html)

#![deny(warnings)]

extern crate xmltree;

use std::ops::Deref;

use xmltree::Element;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

mod parse;

/// Parses the contents of a SVD file (XML)
pub fn parse(xml: &str) -> Device {
    Device::parse(xml)
}

trait ElementExt {
    fn get_child_text<K>(&self, k: K) -> Option<String>
        where String: PartialEq<K>;
    fn debug(&self);
}

impl ElementExt for Element {
    fn get_child_text<K>(&self, k: K) -> Option<String>
        where String: PartialEq<K>
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
    pub peripherals: Vec<Peripheral>,
    pub defaults: Defaults,
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
            peripherals: try!(tree.get_child("peripherals"))
                .children
                .iter()
                .map(Peripheral::parse)
                .collect(),
            defaults: Defaults::parse(tree),
        }
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
                .map(|rs| {
                    rs.children
                        .iter()
                        .filter_map(Register::parse)
                        .collect()
                }),
            derived_from: tree.attributes
                .get("derivedFrom")
                .map(|s| s.to_owned()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: u32,
}

impl Interrupt {
    fn parse(tree: &Element) -> Interrupt {
        Interrupt {
            name: try!(tree.get_child_text("name")),
            description: tree.get_child_text("description"),
            value: try!(parse::u32(try!(tree.get_child("value")))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RegisterInfo {
    pub name: String,
    pub description: String,
    pub address_offset: u32,
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    /// `None` indicates that the `<fields>` node is not present
    pub fields: Option<Vec<Field>>,
}

#[derive(Clone, Debug)]
pub struct RegisterArrayInfo {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub enum Register {
    Single(RegisterInfo),
    Array(RegisterInfo, RegisterArrayInfo),
}

impl Deref for Register {
    type Target = RegisterInfo;

    fn deref(&self) -> &RegisterInfo {
        match *self {
            Register::Single(ref info) => info,
            Register::Array(ref info, _) => info,
        }
    }
}

impl RegisterInfo {
    fn parse(tree: &Element) -> RegisterInfo {
        RegisterInfo {
            name: try!(tree.get_child_text("name")),
            description: try!(tree.get_child_text("description")),
            address_offset:
                try!(parse::u32(try!(tree.get_child("addressOffset")))),
            size: tree.get_child("size").map(|t| try!(parse::u32(t))),
            access: tree.get_child("access").map(Access::parse),
            reset_value: tree.get_child("resetValue")
                .map(|t| try!(parse::u32(t))),
            reset_mask: tree.get_child("resetMask")
                .map(|t| try!(parse::u32(t))),
            fields: tree.get_child("fields")
                .map(|fs| fs.children.iter().map(Field::parse).collect()),
        }
    }
}

impl RegisterArrayInfo {
    fn parse(tree: &Element) -> RegisterArrayInfo {
        RegisterArrayInfo {
            dim: try!(tree.get_child_text("dim").unwrap().parse::<u32>()),
            dim_increment: try!(tree.get_child("dimIncrement")
                .map(|t| try!(parse::u32(t)))),
            dim_index: tree.get_child("dimIndex")
                .map(|c| parse::dim_index(try!(c.text.as_ref()))),
        }
    }
}

impl Register {
    // TODO handle "clusters", return `Register` not an `Option`
    fn parse(tree: &Element) -> Option<Register> {
        if tree.name == "cluster" {
            return None;
        }

        assert_eq!(tree.name, "register");

        let info = RegisterInfo::parse(tree);

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterArrayInfo::parse(tree);
            assert!(info.name.contains("%s"));
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Some(Register::Array(info, array_info))
        } else {
            Some(Register::Single(info))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Access {
    ReadOnly,
    ReadWrite,
    ReadWriteOnce,
    WriteOnce,
    WriteOnly,
}

impl Access {
    fn parse(tree: &Element) -> Access {
        let text = try!(tree.text.as_ref());

        match &text[..] {
            "read-only" => Access::ReadOnly,
            "read-write" => Access::ReadWrite,
            "read-writeOnce" => Access::ReadWriteOnce,
            "write-only" => Access::WriteOnly,
            "writeOnce" => Access::WriteOnce,
            _ => panic!("unknown access variant: {}", text),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub bit_range: BitRange,
    pub access: Option<Access>,
    pub enumerated_values: Vec<EnumeratedValues>,
}

impl Field {
    fn parse(tree: &Element) -> Field {
        assert_eq!(tree.name, "field");

        Field {
            name: try!(tree.get_child_text("name")),
            description: tree.get_child_text("description"),
            bit_range: BitRange::parse(tree),
            access: tree.get_child("access").map(Access::parse),
            enumerated_values: tree.children
                .iter()
                .filter(|t| t.name == "enumeratedValues")
                .map(EnumeratedValues::parse)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BitRange {
    pub offset: u32,
    pub width: u32,
}

impl BitRange {
    fn parse(tree: &Element) -> BitRange {
        let (end, start): (u32, u32) =
            if let Some(range) = tree.get_child("bitRange") {
                let text = try!(range.text.as_ref());

                assert!(text.starts_with('['));
                assert!(text.ends_with(']'));

                let mut parts = text[1..text.len() - 1].split(':');

                (try!(try!(parts.next()).parse()),
                 try!(try!(parts.next()).parse()))
            } else if let (Some(lsb), Some(msb)) =
                (tree.get_child_text("lsb"), tree.get_child_text("msb")) {
                (try!(msb.parse()), try!(lsb.parse::<u32>()))
            } else {
                return BitRange {
                    offset: try!(try!(tree.get_child_text("bitOffset"))
                        .parse()),
                    width: try!(try!(tree.get_child_text("bitWidth")).parse()),
                };
            };

        BitRange {
            offset: start,
            width: end - start + 1,
        }
    }
}

/// Register default properties
#[derive(Clone, Copy, Debug)]
pub struct Defaults {
    pub size: Option<u32>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    pub access: Option<Access>,
}

impl Defaults {
    fn parse(tree: &Element) -> Defaults {
        Defaults {
            size: tree.get_child("size").map(|t| try!(parse::u32(t))),
            reset_value: tree.get_child("resetValue")
                .map(|t| try!(parse::u32(t))),
            reset_mask: tree.get_child("resetMask")
                .map(|t| try!(parse::u32(t))),
            access: tree.get_child("access").map(Access::parse),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    Read,
    Write,
    ReadWrite,
}

impl Usage {
    fn parse(tree: &Element) -> Usage {
        let text = try!(tree.text.as_ref());

        match &text[..] {
            "read" => Usage::Read,
            "write" => Usage::Write,
            "read-write" => Usage::ReadWrite,
            _ => panic!("unknown usage variant: {}", text),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumeratedValues {
    pub name: Option<String>,
    pub usage: Option<Usage>,
    pub derived_from: Option<String>,
    pub values: Vec<EnumeratedValue>,
}

impl EnumeratedValues {
    fn parse(tree: &Element) -> EnumeratedValues {
        assert_eq!(tree.name, "enumeratedValues");

        EnumeratedValues {
            name: tree.get_child_text("name"),
            usage: tree.get_child("usage").map(Usage::parse),
            derived_from: tree.attributes
                .get(&"derivedFrom".to_owned())
                .map(|s| s.to_owned()),
            values: tree.children
                .iter()
                .filter_map(EnumeratedValue::parse)
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumeratedValue {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<u32>,
    pub is_default: Option<bool>,
}

impl EnumeratedValue {
    fn parse(tree: &Element) -> Option<EnumeratedValue> {
        if tree.name != "enumeratedValue" {
            return None;
        }

        Some(EnumeratedValue {
            name: try!(tree.get_child_text("name")),
            description: tree.get_child_text("description"),
            value: tree.get_child("value").map(|t| try!(parse::u32(t))),
            is_default: tree.get_child_text("isDefault")
                .map(|t| try!(t.parse())),
        })
    }
}
