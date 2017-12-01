#![feature(custom_attribute)]
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

// TEMP#![deny(warnings)]


extern crate either;
extern crate xmltree;
#[macro_use]
extern crate failure;

use std::ops::Deref;

use either::Either;
use xmltree::Element;

use failure::{Error, err_msg, ResultExt};
mod parse;
pub mod errors;


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
    fn get_self_text(&self) -> Result<&String, Error>;
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
            Some(val) => Ok(Some(
                    val.text.clone()
                    .ok_or(
                        errors::TagError::EmptyTag {
                            name: format!("{}", k).to_owned(),
                            content: errors::XmlContent::Text
                        }
                    )?
            )),
       } 
    }
    fn get_child_text<K>(&self, k: K) -> Result<String, Error>
    where
        String: PartialEq<K>,
        K: std::fmt::Display + Clone,
    {
        self.get_child_text_opt(k.clone())?
            .ok_or(
                errors::TagError::MissingTag { name: format!("{}", k).to_owned() }.into() 
            ) 
    }

    fn get_child_res<K>(&self, k: K) -> Result<&Element, Error>
    where
        String: PartialEq<K>,
        K: std::fmt::Display + Clone,
    {
        if let Some(res) = self.get_child(k.clone()) {
            return Ok(res)
        } else {
            Err(errors::TagError::EmptyTag {
                    name: format!("{}", k).to_owned(),
                    content: errors::XmlContent::Unknown
                }.into())
        }
    }
    
    fn get_self_text(&self) -> Result<&String, Error> {
        self.text.as_ref().ok_or(errors::TagError::EmptyTag {
            name: self.name.clone(),
            content: errors::XmlContent::Text
        }.into())
    }

    fn debug(&self) {
        println!("<{}>", self.name);
        for c in &self.children {
            println!("{}: {:?}", c.name, c.text)
        }
        println!("</{}>", self.name);
    }
}

/// Convenience function for elevating errors in optional fields
fn and_then_result<U,E,L, F: std::ops::Fn(L) -> Result<U,E>>(opt: Option<L>, f: F) -> Result<Option<U>,E> {
    if let Some(k) = opt {
        match f(k) {    
            Ok(u) => Ok(Some(u)),
            Err(e) => Err(e),
        }
    } else {
        Ok(None)
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
    pub fn parse(svd: &str) -> Result<Device,Error> {
        let tree = &Element::parse(svd.as_bytes())?;
        let peripherals = {
            let res: Result<Vec<_>, _> = tree.get_child_res("peripherals")?.children
                .iter()
                .enumerate()
                .map(|(i,p)| Peripheral::parse(p).map_err(|e| (i+1,e)))
                .collect();
            
            res.map_err(|err| errors::PeripheralError::from_cause(err.1, err.0))?
        };
        Ok(Device {
            name: tree.get_child_text("name")?,
            cpu: {
                if let Some(res) = tree.get_child("cpu").map(Cpu::parse) {
                    Some(res?)
                } else {
                    None
                }
            },
            peripherals,
            defaults: Defaults::parse(tree)?,
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
        let text = tree.get_self_text()?;

        Ok(match &text[..] {
            "little" => Endian::Little,
            "big" => Endian::Big,
            "selectable" => Endian::Selectable,
            "other" => Endian::Other,
            _ => return Err(errors::EndianVariantError(text.clone()).into()),
        })
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
        let name = tree.get_child_text("name")?; 
        Peripheral::_parse(tree, name.clone()).map_err(|e| errors::Named(name, e).into())
    }
    fn _parse(tree: &Element, name: String) -> Result<Peripheral,Error> {
        if tree.name != "peripheral" {
            return Err(format_err!("Expected perhipheral tag"))
        }

        Ok(Peripheral {
            name,
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
                        .enumerate()
                        .map(|(i,p)| cluster_register_parse(p).map_err(|e| (i+1,e)))
                        .collect();
                    Some(
                        res.map_err(|err|
                                    errors::RegisterError::from_cause(err.1, err.0)
                                    )?
                        )
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
            size: and_then_result(tree.get_child("size"), parse::u32)?,
            access: {
                if let Some(access) = tree.get_child("access") {
                    Some(Access::parse(access)?)
                } else {
                    None
                }
            },
            reset_value:
                and_then_result(tree.get_child("resetValue"), parse::u32)?,
            reset_mask:
                and_then_result(tree.get_child("resetMask"), parse::u32)?,
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
        let name = tree.get_child_text("name")?; 
        RegisterInfo::_parse(tree, name.clone()).map_err(|e| errors::Named(name, e).into())
    }
    fn _parse(tree: &Element,name: String) -> Result<RegisterInfo, Error> {
        Ok(RegisterInfo {
            name: name,
            description: tree.get_child_text("description")?,
            address_offset: parse::u32(tree.get_child_res("addressOffset")?)?,
            size: and_then_result(tree.get_child("size"), parse::u32)?,
            access: and_then_result(tree.get_child("access"),Access::parse)?,
            reset_value:
                and_then_result(tree.get_child("resetValue"), parse::u32)?,
            reset_mask:
                and_then_result(tree.get_child("resetMask"), parse::u32)?,
            fields: {
                if let Some(rs) = tree.get_child("fields") {
                    let res: Result<Vec<_>, _> = rs.children
                        .iter()
                        .enumerate()
                        .map(|(i,p)| Field::parse(p).map_err(|e| (i+1,e)))
                        .collect();
                    Some(
                            res.map_err(|err|
                                errors::FieldError::from_cause(err.1, err.0)
                            )?
                        )
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
            dim_index: and_then_result(tree.get_child("dimIndex"), parse::dim_index)?,
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
        let text = tree.get_self_text()?;
        Ok(match &text[..] {
            "read-only" => Access::ReadOnly,
            "read-write" => Access::ReadWrite,
            "read-writeOnce" => Access::ReadWriteOnce,
            "write-only" => Access::WriteOnly,
            "writeOnce" => Access::WriteOnce,
            _ => return Err(errors::AccessVariantError(text.clone()).into()),
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
        let name = tree.get_child_text("name")?;
        Field::_parse(tree, name.clone()).map_err(|e| errors::Named(name, e).into())
        
    }
    fn _parse(tree: &Element, name: String) -> Result<Field,Error> {

        Ok(Field {
            name,
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
        BitRange::_parse(tree).map_err(|e| errors::BitRangeError::from_cause(e)).map_err(|e| e.into())
    }
    fn _parse(tree: &Element) -> Result<BitRange, Error> {
        let (end, start): (u32, u32) = if let Some(range) =
            tree.get_child_text_opt("bitRange")? {

            if !range.starts_with('[') {
                return Err(errors::BitRangeParseError::MissingOpen.into())
            }
            if !range.ends_with(']') {
                return Err(errors::BitRangeParseError::MissingClose.into())
            }

            let mut parts = range[1..range.len() - 1].split(':');

            // FIXME: This error can be much better
            (
                parts.next().ok_or(errors::BitRangeParseError::Syntax)?.parse::<u32>().map_err(|e| errors::BitRangeParseError::ParseError(e))?,
                parts.next().ok_or(errors::BitRangeParseError::Syntax)?.parse::<u32>().map_err(|e| errors::BitRangeParseError::ParseError(e))?
            )
        } else if let (Some(lsb), Some(msb)) =
            (tree.get_child("lsb"), tree.get_child("msb")) {
            (parse::u32(msb).with_context(|e| errors::BitRangeError::MsbLsb)?, parse::u32(lsb).with_context(|e| errors::BitRangeError::MsbLsb)?)
        } else { // FIXME: This branch should not be the end condition, an error should be.
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
                            parse::bool(tree.get_child_res(field.as_ref())?)?
                    ))
                }
                "useEnumeratedValues" => {
                    Ok(WriteConstraint::UseEnumeratedValues(
                            parse::bool(tree.get_child_res(field.as_ref())?)?
                    ))
                }
                "range" => {
                    Ok(WriteConstraint::Range(
                        WriteConstraintRange::parse(tree.get_child_res(field.as_ref())?)?
                    ))
                }
                _ => Err(errors::WriteConstraintError::Variant(field.clone()).into()),
            }
        } else {
            Err(errors::WriteConstraintError::TooManyElements.into())
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
    fn parse(tree: &Element) -> Result<Defaults, Error> {
        Ok(Defaults {
            size: and_then_result(tree.get_child("size"), parse::u32)?,
            reset_value:
                and_then_result(tree.get_child("resetValue"),parse::u32)?,
            reset_mask:
                and_then_result(tree.get_child("resetMask"), parse::u32)?,
            access: and_then_result(tree.get_child("access"), Access::parse)?,
            _extensible: (),
        })
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
        let text = tree.get_self_text()?;
        Ok(match &text[..] {
            "read" => Usage::Read,
            "write" => Usage::Write,
            "read-write" => Usage::ReadWrite,
            _ => return Err(errors::UsageVariantError(text.clone()).into()),
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
                    .enumerate()
                    .map(|(i,v)| EnumeratedValue::parse(v).map_err(|e| (i+1,e)))
                    .collect();
                // Unwrap is safe because we filtered all the None
                res.map_err(|err|
                            errors::EnumeratedValueError::from_cause(err.1,err.0))?.into_iter().filter(|r| r.is_some()).map(|s| s.unwrap()).collect()
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
        let name = tree.get_child_text("name")?;
        EnumeratedValue::_parse(tree, name.clone()).map_err(|e| errors::Named(name, e).into()).map(|val| Some(val))
        //Peripheral::_parse(tree, name.clone())
    }
    fn _parse(tree: &Element, name: String) -> Result<EnumeratedValue, Error> {
        Ok(
            EnumeratedValue {
                name, // FIXME: Capture error
                description: tree.get_child_text_opt("description")?,
                value: and_then_result(tree.get_child("value"), parse::u32)?,
                is_default: tree.get_child_text_opt("isDefault")?.map(|t| t.parse().unwrap()),
                _extensible: (),
            },
        )
    }
}
