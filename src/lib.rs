#![feature(try_trait)]
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
//! }cro_use] extern crate failure_derive;u
//! ```
//!
//! # References
//!
//! - [SVD Schema file](https://www.keil.com/pack/doc/CMSIS/SVD/html/schema_1_2_gr.html)
//! - [SVD file database](https://github.com/posborne/cmsis-svd/tree/master/data)
//! - [Sample SVD file](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_Example_pg.html)

// TEMP#![deny(warnings)]

extern crate either;
extern crate xmltree;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::ops::Deref;

use either::Either;
use xmltree::Element;

use failure::{Error,err_msg, ResultExt};
mod parse;


/// Parses the contents of a SVD file (XML)
pub fn parse(xml: &str) -> Result<Device,Error> {
    Device::parse(xml)
}

trait ElementExt {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>, Error>
    where
        String: PartialEq<K>,
        K: std::fmt::Display + Clone;
    fn get_child_text<K>(&self, k: K) -> Result<String, Error>
    where
        String: PartialEq<K>,
        K: std::fmt::Display + Clone;
    fn get_child_res<K>(&self, k: K) -> Result<&Element, Error>
    where
        String: PartialEq<K>,
        K: std::fmt::Display + Clone;
    fn debug(&self);
}

impl ElementExt for Element {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>, Error>
    where
        String: PartialEq<K>,
        K: std::fmt::Display + Clone,
    {
       match self.get_child(k.clone()) {
            None => Ok(None),
            Some(val) => Ok(Some(val.text.clone().ok_or(format_err!("Couldn't get `<{}>` tag", k))?)),
       } 
    }
    fn get_child_text<K>(&self, k: K) -> Result<String, Error>
    where
        String: PartialEq<K>,
        K: std::fmt::Display + Clone,
    {
        self.get_child_text_opt(k.clone())?.ok_or(format_err!("Expected a `<{}>` tag but found none", k)) 
    }

    fn get_child_res<K>(&self, k: K) -> Result<&Element, Error>
    where
        String: PartialEq<K>,
        K: std::fmt::Display + Clone,
    {
        if let Some(res) = self.get_child(k.clone()) {
            return Ok(res)
        } else {
            Err(err_msg(format!("Couldn't get a `<{}>` tag", k)))
        }
    }
    

    fn debug(&self) {
        println!("<{}>", self.name);
        for c in &self.children {
            println!("{}: {:?}", c.name, c.text)
        }
        println!("</{}>", self.name);
    }
}

/*impl std::convert::From<std::option::NoneError> for Error {
    fn from(error: std::option::NoneError) -> Self {
        NewNoneError
    }
}*/
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
    pub fn parse(svd: &str) -> Result<Device,Error> {
        let tree = &Element::parse(svd.as_bytes())?;
        let peripherals = {
            // FIXME: Show peripheral number in error
            let res: Result<Vec<_>, _> = tree.get_child_res("peripherals")?.children
                .iter()
                .map(Peripheral::parse)
                .collect();
            
            res?
        };
        Ok(Device {
            name: tree.get_child_text("name")?, // FIXME: Should capture the caused
            cpu: {
                if let Some(res) = tree.get_child("cpu").map(Cpu::parse) {
                    Some(res?)
                } else {
                    None
                }
            },
            peripherals,
            defaults: Defaults::parse(tree),
            _extensible: (),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
    Selectable,
    Other
}

impl Endian {
    fn parse(tree: &Element) -> Result<Endian,Error> {
        let text = tree.text.as_ref().ok_or(err_msg("couldnt get endian"))?; // FIXME: Endian::parse should really take a str

        match &text[..] {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            "selectable" => Ok(Endian::Selectable),
            "other" => Ok(Endian::Other),
            _ => Err(format_err!("unknown endian variant: {}", text)),
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
    fn parse(tree: &Element) -> Result<Cpu,Error> {
        if tree.name != "cpu" {
            return Err(format_err!("Expected cpu tag")) // FIXME: msg
        }

        Ok(Cpu {
            name: tree.get_child_text("name")?, // FIXME: Capture error
            revision: tree.get_child_text("revision")?, // FIXME: Capture error
            endian: Endian::parse(tree.get_child_res("endian")?)?, // FIXME: Capture error
            mpu_present: parse::bool(tree.get_child_res("mpuPresent")?)?, // FIXME: Capture errors
            fpu_present: parse::bool(tree.get_child_res("fpuPresent")?)?, // FIXME: Capture errors
            nvic_priority_bits:
                parse::u32(tree.get_child_res("nvicPrioBits")?)?, // FIXME: Capture errors
            has_vendor_systick:
                parse::bool(tree.get_child_res("vendorSystickConfig")?)?, // FIXME: Capture errors

            _extensible: (),
        })
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
    pub registers: Option<Vec<Either<Register, Cluster>>>,
    pub derived_from: Option<String>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Peripheral {
    fn parse(tree: &Element) -> Result<Peripheral,Error> {
        if tree.name != "peripheral" {
            return Err(format_err!("Expected perhipheral tag"))
        }

        Ok(Peripheral {
            name: tree.get_child_text("name")?, // FIXME: Capture error
            group_name: tree.get_child_text_opt("groupName")?,
            description: tree.get_child_text_opt("description")?,
            base_address: parse::u32(tree.get_child_res("baseAddress")?)?,
            interrupt: {
                let res: Result<Vec<_>, _> = tree.children
                    .iter()
                    .filter(|t| t.name == "interrupt")
                    .map(Interrupt::parse)
                    .collect();
                res?
            },
            registers: {
                if let Some(rs) = tree.get_child("registers") {
                    let res: Result<Vec<_>, _> = rs.children
                        .iter()
                        .map(cluster_register_parse)
                        .collect();
                    Some(res?)
                } else {
                    None
                }
            },
            derived_from: tree.attributes.get("derivedFrom").map(
                |s| {
                    s.to_owned()
                },
            ),
            _extensible: (),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: u32,
}

impl Interrupt {
    fn parse(tree: &Element) -> Result<Interrupt, Error> {
        Ok(Interrupt {
            name: tree.get_child_text("name")?, // FIXME: Capture error
            description: tree.get_child_text_opt("description")?,
            value: parse::u32(tree.get_child_res("value")?)?,
        })
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

fn cluster_register_parse(tree: &Element) -> Result<Either<Register, Cluster>, Error> {
    if tree.name == "register" {
        Ok(Either::Left(Register::parse(tree)?))
    } else if tree.name == "cluster" {
        Ok(Either::Right(Cluster::parse(tree)?))
    } else {
        unreachable!()
    }
}

impl Cluster {
    fn parse(tree: &Element) -> Result<Cluster, Error> {
        assert_eq!(tree.name, "cluster");

        let info = ClusterInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterClusterArrayInfo::parse(tree)?;
            assert!(info.name.contains("%s")); // FIXME: return as Result
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len()) // FIXME: Return as Result
            }
            Ok(Cluster::Array(info, array_info))
        } else {
            Ok(Cluster::Single(info))
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
    fn parse(tree: &Element) -> Result<ClusterInfo,Error> {
        Ok(ClusterInfo {
            name: tree.get_child_text("name")?, // FIXME: Capture error
            description: tree.get_child_text("description")?, // FIXME: Capture error
            header_struct_name: tree.get_child_text_opt("headerStructName")?,
            address_offset:
                parse::u32(tree.get_child_res("addressOffset")?)?, // FIXME: Capture errors
            size: tree.get_child_res("size").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            access: {
                if let Some(access) = tree.get_child("access") {
                    Some(Access::parse(access)?)
                } else {
                    None
                }
            },
            reset_value:
                tree.get_child_res("resetValue").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            reset_mask:
                tree.get_child_res("resetMask").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            children: {
                let res: Result<Vec<_>, _> = tree.children
                    .iter()
                    .filter(|t| t.name == "register" || t.name == "cluster")
                    .map(cluster_register_parse)
                    .collect();
                res?
            },

            _extensible: (),
        })
    }
}

impl RegisterInfo {
    fn parse(tree: &Element) -> Result<RegisterInfo, Error> {
        Ok(RegisterInfo {
            name: tree.get_child_text("name")?, // FIXME: Capture error
            description: tree.get_child_text("description")?, // FIXME: Capture error
            address_offset: parse::u32(tree.get_child_res("addressOffset")?)?,
            size: tree.get_child_res("size").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            access: {
                if let Some(access) = tree.get_child("access") {
                    Some(Access::parse(access)?)
                } else {
                    None
                }
            }, 
            reset_value:
                tree.get_child_res("resetValue").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            reset_mask:
                tree.get_child_res("resetMask").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            fields: {
                if let Some(rs) = tree.get_child("fields") {
                    let res: Result<Vec<_>, _> = rs.children
                        .iter()
                        .map(Field::parse)
                        .collect();
                    Some(res?)
                } else {
                    None
                }
            },
            write_constraint: {
                if let Some(write_constraint) = tree.get_child("writeConstraint") {
                    Some(WriteConstraint::parse(write_constraint)?)
                } else {
                    None
                }
            },
            _extensible: (),
        })
    }
}

impl RegisterClusterArrayInfo {
    fn parse(tree: &Element) -> Result<RegisterClusterArrayInfo, Error> {
        Ok(RegisterClusterArrayInfo {
            dim: tree.get_child_text("dim")?.parse::<u32>()?, // FIXME: Capture error
            dim_increment: parse::u32(tree.get_child_res("dimIncrement")?)?, // FIXME: Capture error
            dim_index: {
                if let Some(res) = tree.get_child("dimIndex").map(|c| parse::dim_index(c.text.as_ref().ok_or(format_err!("couldnt get text"))?)) { 
                    // FIXME: Capture error
                    Some(res?)
                } else {
                    None
                }
            },
        })
    }
}

impl Register {
    fn parse(tree: &Element) -> Result<Register, Error> {
        assert_eq!(tree.name, "register"); // FIXME: use if and ?

        let info = RegisterInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterClusterArrayInfo::parse(tree)?;
            assert!(info.name.contains("%s"));
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Ok(Register::Array(info, array_info))
        } else {
            Ok(Register::Single(info))
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
    fn parse(tree: &Element) -> Result<Access,Error> {
        let text = tree.text.as_ref().ok_or(err_msg("couldnt get access"))?; // FIXME: Endian::parse should really take a str
        Ok(match &text[..] {
            "read-only" => Access::ReadOnly,
            "read-write" => Access::ReadWrite,
            "read-writeOnce" => Access::ReadWriteOnce,
            "write-only" => Access::WriteOnly,
            "writeOnce" => Access::WriteOnce,
            _ => panic!("unknown access variant: {}", text), // FIXME: use result
        })
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
    fn parse(tree: &Element) -> Result<Field,Error> {
        assert_eq!(tree.name, "field"); // FIXME: Use if and ?

        Ok(Field {
            name: tree.get_child_text("name")?, // FIXME: Capture error
            description: tree.get_child_text_opt("description")?,
            bit_range: BitRange::parse(tree)?,
            access: {
                if let Some(access) = tree.get_child("access") {
                    Some(Access::parse(access)?)
                } else {
                    None
                }
            },  
            enumerated_values: {
                let res: Result<Vec<_>, _> = tree.children
                    .iter()
                    .filter(|t| t.name == "enumeratedValues")
                    .map(EnumeratedValues::parse)
                    .collect();
                res?
            },
            write_constraint: {
                if let Some(write_constraint) = tree.get_child("writeConstraint") {
                    Some(WriteConstraint::parse(write_constraint)?)
                } else {
                    None
                }
            },  
            _extensible: (),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BitRange {
    pub offset: u32,
    pub width: u32,
}

impl BitRange {
    fn parse(tree: &Element) -> Result<BitRange, Error> {
        let (end, start): (u32, u32) = if let Some(range) =
            tree.get_child("bitRange") {

            let text = tree.text.as_ref().ok_or(err_msg("couldnt get bitrange"))?; // FIXME: BitRange::parse should really take a str
            assert!(text.starts_with('[')); // FIXME: Use if and format_err!
            assert!(text.ends_with(']')); // FIXME: Use if and format_err!

            let mut parts = text[1..text.len() - 1].split(':');

            (parts.next().ok_or(err_msg("Couldn't get next"))?.parse()?, parts.next().ok_or(err_msg("Couldn't get next"))?.parse()?)
        } else if let (Some(lsb), Some(msb)) =
            (tree.get_child("lsb"), tree.get_child("msb")) {
            (parse::u32(msb)?, parse::u32(lsb)?)
        } else {
            return Ok(BitRange {
                       offset: parse::u32(tree.get_child_res("bitOffset")?)?, // FIXME: Capture errors
                       width: parse::u32(tree.get_child_res("bitWidth")?)?, // FIXME: Capture errors
                   });
        };

        Ok(BitRange {
            offset: start,
            width: end - start + 1,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WriteConstraintRange {
    pub min: u32,
    pub max: u32,
}

impl WriteConstraintRange {
    fn parse(tree: &Element) -> Result<WriteConstraintRange, Error> {
        Ok(WriteConstraintRange {
            min: tree.get_child_text("minimum")?.parse()?, // FIXME: Capture errors
            max: tree.get_child_text("maximum")?.parse()?, // FIXME: Capture errors
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WriteConstraint {
    WriteAsRead(bool),
    UseEnumeratedValues(bool),
    Range(WriteConstraintRange),
}

impl WriteConstraint {
    fn parse(tree: &Element) -> Result<WriteConstraint, Error> {
        if tree.children.len() == 1 {
            let ref field = tree.children[0].name;
            // Write constraint can only be one of the following
            match field.as_ref() {
                "writeAsRead" => {
                    Ok(WriteConstraint::WriteAsRead(
                            tree.get_child(field.as_ref())
                                .and_then(|t| parse::bool(t).ok()).ok_or(err_msg("writeAsRead"))? // FIXME: Capture errors, and fix silencing
                    ))
                }
                "useEnumeratedValues" => {
                    Ok(WriteConstraint::UseEnumeratedValues(
                            tree.get_child(field.as_ref())
                                .and_then(|t| parse::bool(t).ok()).ok_or(err_msg("useEnumeratedValues"))? // FIXME: Capture errors, and fix silencing
                    ))
                }
                "range" => {
                    Ok(WriteConstraint::Range(
                        // FIXME: Capture error
                        WriteConstraintRange::parse(tree.get_child_res(field.as_ref())?)?
                    ))
                }
                v => return Err(format_err!("unknown <writeConstraint> variant: {}", v)),
            }
        } else {
            return Err(format_err!("found more than one <WriteConstraint> element"))
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
            size: tree.get_child_res("size").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            reset_value:
                tree.get_child_res("resetValue").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            reset_mask:
                tree.get_child_res("resetMask").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
            access: tree.get_child_res("access").and_then(Access::parse).ok(), // FIXME: Silences parsing errors
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
    fn parse(tree: &Element) -> Result<Usage, Error> {
        let text = tree.text.as_ref().ok_or(err_msg("couldnt get usage"))?; // FIXME: BitRange::parse should really take a str

        Ok(match &text[..] {
            "read" => Usage::Read,
            "write" => Usage::Write,
            "read-write" => Usage::ReadWrite,
            _ => panic!("unknown usage variant: {}", text),
        })
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
    fn parse(tree: &Element) -> Result<EnumeratedValues,Error> {
        assert_eq!(tree.name, "enumeratedValues");

        Ok(EnumeratedValues {
            name: tree.get_child_text_opt("name")?,
            usage: {
                if let Some(usage) = tree.get_child("usage") {
                    Some(Usage::parse(usage)?)
                } else {
                    None
                }
            },
            derived_from: tree.attributes
                .get(&"derivedFrom".to_owned())
                .map(|s| s.to_owned()),
            values: {
                let res: Result<Vec<_>, _> = tree.children
                    .iter()
                    .map(EnumeratedValue::parse)
                    .collect();
                // Unwrap is safe
                res?.into_iter().filter(|r| r.is_some()).map(|s| s.unwrap()).collect()
            },
            _extensible: (),
        })
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
    fn parse(tree: &Element) -> Result<Option<EnumeratedValue>, Error> {
        if tree.name != "enumeratedValue" {
            return Ok(None);
        }

        Ok(Some(
            EnumeratedValue {
                name: tree.get_child_text("name")?, // FIXME: Capture error
                description: tree.get_child_text_opt("description")?,
                value: tree.get_child_res("value").and_then(|t| parse::u32(t)).ok(), // FIXME: Silences parsing errors
                is_default: tree.get_child_text_opt("isDefault")?.map( // Silences error
                    |t| {
                        t.parse().unwrap() // FIXME: Make into error
                    },
                ),
                _extensible: (),
            },
        ))
    }
}
