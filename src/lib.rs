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

extern crate either;
extern crate xmltree;

use std::ops::Deref;

use either::Either;
use xmltree::Element;

pub mod svd;
use svd::cpu::Cpu;
use svd::interrupt::Interrupt;
use svd::access::Access;
use svd::bitrange::BitRange;
use svd::writeconstraint::{WriteConstraint};

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

pub mod error;
use error::SVDError;
pub mod parse;
pub mod types;
use types::Parse;


/// Parses the contents of a SVD file (XML)
pub fn parse(xml: &str) -> Result<Device, SVDError> {
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
    pub fn parse(svd: &str) -> Result<Device, SVDError> {
        let tree = &try!(Element::parse(svd.as_bytes()));

        Ok(Device {
            name: try!(tree.get_child_text("name")),
            cpu: Some(Cpu::parse(parse::get_child_elem("cpu", tree)?)?),
            peripherals: try!(tree.get_child("peripherals"))
                .children
                .iter()
                .map(Peripheral::parse)
                .collect(),
            defaults: Defaults::parse(tree),
            _extensible: (),
        })
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
    pub registers: Option<Vec<Either<Register, Cluster>>>,
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
                .map(|i| Interrupt::parse(i).ok() )
                .filter(|i| i.is_some() )
                .map(|i| i.unwrap() )
                .collect::<Vec<_>>(),
            registers: tree.get_child("registers").map(|rs| {
                rs.children.iter().map(cluster_register_parse).collect()
            }),
            derived_from: tree.attributes.get("derivedFrom").map(
                |s| {
                    s.to_owned()
                },
            ),
            _extensible: (),
        }
    }
}



#[derive(Clone, Debug)]
pub struct ClusterInfo {
    pub name: String,
    pub description: String,
    pub header_struct_name: Option<String>,
    pub address_offset: u32,
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    pub children: Vec<Either<Register, Cluster>>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

#[derive(Clone, Debug)]
pub struct RegisterInfo {
    pub name: String,
    pub alternate_group: Option<String>,
    pub alternate_register: Option<String>,
    pub derived_from: Option<String>,
    pub description: String,
    pub address_offset: u32,
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    /// `None` indicates that the `<fields>` node is not present
    pub fields: Option<Vec<Field>>,
    pub write_constraint: Option<WriteConstraint>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

#[derive(Clone, Debug)]
pub struct RegisterClusterArrayInfo {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
}

fn cluster_register_parse(tree: &Element) -> Either<Register, Cluster> {
    if tree.name == "register" {
        Either::Left(Register::parse(tree))
    } else if tree.name == "cluster" {
        Either::Right(Cluster::parse(tree))
    } else {
        unreachable!()
    }
}

impl Cluster {
    fn parse(tree: &Element) -> Cluster {
        assert_eq!(tree.name, "cluster");

        let info = ClusterInfo::parse(tree);

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterClusterArrayInfo::parse(tree);
            assert!(info.name.contains("%s"));
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Cluster::Array(info, array_info)
        } else {
            Cluster::Single(info)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Cluster {
    Single(ClusterInfo),
    Array(ClusterInfo, RegisterClusterArrayInfo),
}

impl Deref for Cluster {
    type Target = ClusterInfo;

    fn deref(&self) -> &ClusterInfo {
        match *self {
            Cluster::Single(ref info) => info,
            Cluster::Array(ref info, _) => info,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Register {
    Single(RegisterInfo),
    Array(RegisterInfo, RegisterClusterArrayInfo),
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

impl ClusterInfo {
    fn parse(tree: &Element) -> ClusterInfo {
        ClusterInfo {
            name: try!(tree.get_child_text("name")),
            description: try!(tree.get_child_text("description")),
            header_struct_name: tree.get_child_text("headerStructName"),
            address_offset: {
                try!(parse::u32(try!(tree.get_child("addressOffset"))))
            },
            size: tree.get_child("size").map(|t| try!(parse::u32(t))),
            //access: tree.get_child("access").map(|t| Access::parse(t).ok() ),
            access: parse::optional("access", tree, Access::parse).unwrap(),
            reset_value:
                tree.get_child("resetValue").map(|t| try!(parse::u32(t))),
            reset_mask:
                tree.get_child("resetMask").map(|t| try!(parse::u32(t))),
            children: tree.children
                .iter()
                .filter(|t| t.name == "register" || t.name == "cluster")
                .map(cluster_register_parse)
                .collect(),
            _extensible: (),
        }
    }
}

impl RegisterInfo {
    fn parse(tree: &Element) -> RegisterInfo {
        RegisterInfo {
            name: try!(tree.get_child_text("name")),
            alternate_group: tree.get_child_text("alternateGroup"),
            alternate_register: tree.get_child_text("alternateRegister"),
            derived_from: tree.attributes.get("derivedFrom").map(|s| s.to_owned()),
            description: try!(tree.get_child_text("description")),
            address_offset: {
                try!(parse::u32(try!(tree.get_child("addressOffset"))))
            },
            size: tree.get_child("size").map(|t| try!(parse::u32(t))),
            access: parse::optional("access", tree, Access::parse).unwrap(),
            reset_value:
                tree.get_child("resetValue").map(|t| try!(parse::u32(t))),
            reset_mask:
                tree.get_child("resetMask").map(|t| try!(parse::u32(t))),
            fields:
                tree.get_child("fields")
                    .map(|fs| fs.children.iter().map(Field::parse).collect()),
            write_constraint: parse::optional("writeConstraint", tree, WriteConstraint::parse).unwrap(),
            _extensible: (),
        }
    }
}

impl RegisterClusterArrayInfo {
    fn parse(tree: &Element) -> RegisterClusterArrayInfo {
        RegisterClusterArrayInfo {
            dim: try!(tree.get_child_text("dim").unwrap().parse::<u32>()),
            dim_increment: try!(tree.get_child("dimIncrement").map(|t| {
                try!(parse::u32(t))
            })),
            dim_index: tree.get_child("dimIndex").map(|c| {
                parse::dim_index(try!(c.text.as_ref()))
            }),
        }
    }
}

impl Register {
    fn parse(tree: &Element) -> Register {
        assert_eq!(tree.name, "register");

        let info = RegisterInfo::parse(tree);

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterClusterArrayInfo::parse(tree);
            assert!(info.name.contains("%s"));
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Register::Array(info, array_info)
        } else {
            Register::Single(info)
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
    pub write_constraint: Option<WriteConstraint>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Field {
    fn parse(tree: &Element) -> Field {
        assert_eq!(tree.name, "field");

        Field {
            name: try!(tree.get_child_text("name")),
            description: tree.get_child_text("description"),
            bit_range: BitRange::parse(tree).unwrap(),
            access: parse::optional("access", tree, Access::parse).unwrap(),
            enumerated_values: tree.children
                .iter()
                .filter(|t| t.name == "enumeratedValues")
                .map(EnumeratedValues::parse)
                .collect::<Vec<_>>(),
            write_constraint: parse::optional("writeConstraint", tree, WriteConstraint::parse).unwrap(),
            _extensible: (),
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
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Defaults {
    fn parse(tree: &Element) -> Defaults {
        Defaults {
            size: tree.get_child("size").map(|t| try!(parse::u32(t))),
            reset_value:
                tree.get_child("resetValue").map(|t| try!(parse::u32(t))),
            reset_mask:
                tree.get_child("resetMask").map(|t| try!(parse::u32(t))),
            access: parse::optional("access", tree, Access::parse).unwrap(),
            _extensible: (),
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
    // Reserve the right to add more fields to this struct
    _extensible: (),
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
            _extensible: (),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumeratedValue {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<u32>,
    pub is_default: Option<bool>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl EnumeratedValue {
    fn parse(tree: &Element) -> Option<EnumeratedValue> {
        if tree.name != "enumeratedValue" {
            return None;
        }

        Some(
            EnumeratedValue {
                name: try!(tree.get_child_text("name")),
                description: tree.get_child_text("description"),
                value: tree.get_child("value").map(|t| try!(parse::u32(t))),
                is_default: tree.get_child_text("isDefault").map(
                    |t| {
                        try!(t.parse())
                    },
                ),
                _extensible: (),
            },
        )
    }
}
