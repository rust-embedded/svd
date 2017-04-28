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

#[macro_use]
extern crate error_chain;

use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

pub mod errors;

use xmltree::Element;
use errors::{Result, ResultExt};

macro_rules! bail_if_none {
    ($fun:expr, $error:expr) => {
        match $fun {
            ::std::option::Option::Some(val) => val,
            ::std::option::Option::None => bail!($error)
        }
    };
    ($fun:expr, $error:expr,) => {
        match $fun {
            ::std::option::Option::Some(val) => val,
            ::std::option::Option::None => bail!($error)
        }
    };
    ($fun:expr, $error:expr, $($arg:tt)+) => {
        match $fun {
            ::std::option::Option::Some(val) => val,
            ::std::option::Option::None => bail!($error,$($arg)+)
        }
    }
}

mod parse;

/// Parses the contents of a SVD file (XML)
pub fn parse(xml: &str) -> Result<Device> {
    Device::parse(xml).chain_err(|| "Failed to parse svd file")
}

trait ElementExt {
    fn get_child_text_try<K>(&self, k: K) -> Result<Option<String>>
    where
        String: PartialEq<K>,
        K: Debug + Clone;
    fn get_child_try<K>(&self, k: K) -> Result<&Element>
    where
        String: PartialEq<K>,
        K: Debug + Clone;
    fn debug(&self);
}

impl ElementExt for Element {
    fn get_child_text_try<K>(&self, k: K) -> Result<Option<String>>
    where
        String: PartialEq<K>,
        K: Debug + Clone,
    {
        match self.get_child(k.clone()) {
            None => Ok(None),
            Some(val) => Ok(Some(bail_if_none!(val.text.clone(), "Couldn't get `{:?}`", k),),),
        }
    }

    fn get_child_try<K>(&self, k: K) -> Result<&Element>
    where
        String: PartialEq<K>,
        K: Debug + Clone,
    {
        let elem = self.get_child(k.clone());
        elem.ok_or(format!("Failed to get {:?}", k).into())
    }
    fn debug(&self) {
        println!("<{} {:?}>", self.name, self.attributes);
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
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Device {
    /// Parses a SVD file
    ///
    /// # Panics
    ///
    /// If the input is an invalid SVD file (yay, no error handling)
    pub fn parse(svd: &str) -> Result<Device> {
        let tree = &try!(Element::parse(svd.as_bytes()));
        let peripherals_inter = bail_if_none!(
                tree.get_child("peripherals")
                    .map(|rs| rs.children.iter().map(Peripheral::parse)),
                "Failed to get peripherals",
            );
        let mut peripherals = vec![];
        for (i, peripheral_res) in peripherals_inter.enumerate() {
            peripherals.push(
                peripheral_res
                    .chain_err(||
                        format!("Failed to parse peripheral #{}", i+1)
                    )?
                );
        }
        Ok(Device {
            name: bail_if_none!(
                      tree.get_child_text_try("name")?,
                      "Failed to get name of device"
            ),
            peripherals: peripherals, 
            defaults: Defaults::parse(tree)
                .chain_err(|| "Failed to parse defaults")?,
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
    pub registers: Option<Vec<Register>>,
    pub derived_from: Option<String>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Peripheral {
    fn parse(tree: &Element) -> Result<Peripheral> {
        if tree.name != "peripheral" {
            bail!("peripheral not peripheral")
        }
        let name = bail_if_none!(tree.get_child_text_try("name")?, "Couldn't get name");
        let mut registers_inter =
            tree.get_child("registers")
                .map(|rs| rs.children.iter().map(Register::parse));
        let mut registers = Some(vec![]);
        if registers_inter.is_some() {
            for register_res in registers_inter.as_mut().unwrap() {
                if let Some(register) = register_res.chain_err(||
                                format!("Inside peripheral `{}`", name))?
                {
                    registers.as_mut().unwrap().push(register);
                }
            }
        } else {
            registers = None;
        };
        Ok(
            Peripheral {
                name: name,
                group_name: tree.get_child_text_try("groupName")?,
                description: tree.get_child_text_try("description")?,
                base_address:
                    parse::u32(
                        tree.get_child_try("baseAddress")
                            .chain_err(|| "Couldn't get baseAddress")?,
                    ).chain_err(|| "Couldn't parse baseAddress")?,
                interrupt: {
                    let interrupt_inter = tree.children
                        .iter()
                        .filter(|t| t.name == "interrupt")
                        .map(Interrupt::parse);
                    let mut interrupt = vec![];
                    for interrupt_res in interrupt_inter {
                        interrupt.push(interrupt_res?);
                    }
                    interrupt
                },
                registers: registers,
                derived_from: tree.attributes.get("derivedFrom").map(
                    |s| {
                        s.to_owned()
                    },
                ),
                _extensible: (),
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: u32,
}

impl Interrupt {
    fn parse(tree: &Element) -> Result<Interrupt> {
        let name = bail_if_none!(
            tree.get_child_text_try("name")?,
            "Couldn't find name of `interrupt`",
        );
        Ok(
            Interrupt {
                name: name.clone(),
                description: tree.get_child_text_try("description")?,
                value: parse::u32(
                    tree.get_child_try("value")
                        .chain_err(
                            || {
                                format!(
                                    "Couldn't get value of interrupt: `{}`",
                                    name,
                                )
                            },
                        )?,
                )?,
            },
        )
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
    pub write_constraint: Option<WriteConstraint>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
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

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut RegisterInfo {
        match *self {
            Register::Single(ref mut info) => info,
            Register::Array(ref mut info, _) => info,
        }
    }
}

impl RegisterInfo {
    fn parse(tree: &Element) -> Result<RegisterInfo> {
        let name = bail_if_none!(
            tree.get_child_text_try("name")?,
            "Couldn't get name of register.",
        );
        let mut fields_inter =
            tree.get_child("fields")
                .map(|fs| fs.children.iter().map(Field::parse));
        let mut fields = Some(vec![]);
        if fields_inter.is_some() {
            for field_res in fields_inter.as_mut().unwrap() {
                fields
                    .as_mut()
                    .unwrap()
                    .push(
                        field_res.chain_err(
                                || {
                                    format!(
                                        "When trying to parse \
                                data of tag `fields` in register `{}`",
                                        name,
                                    )
                                },
                            )?,
                    );
            }
        } else {
            fields = None;
        };
        Ok(RegisterInfo {
            name: name.clone(), 
            description: bail_if_none!(tree.get_child_text_try("description")?,
                "While getting ´description´ of register `{}`", name),
            address_offset: parse::u32(tree.get_child_try("addressOffset").chain_err(|| format!("Couldn't get `addressOffset` of register: `{}`", name))?)?,
            size: tree.get_child("size").map_or(Ok(None), |t| parse::u32(t).map(|i| Some(i)).chain_err(|| format!("Couldn't parse tag `size` of register: `{}`", name)))?,
            access: tree.get_child("access").map_or(Ok(None), |access| Access::parse(access).map(|ac| Some(ac)).chain_err(|| format!("Couldn't parse tag `access` of register: `{}`", name)))?,
            reset_value: tree.get_child("resetValue").map_or(Ok(None), |t| parse::u32(t).map(|i| Some(i)).chain_err(|| format!("Couldn't parse tag `resetValue` of register: `{}`", name)))?,
            reset_mask: tree.get_child("resetMask").map_or(Ok(None), |t| parse::u32(t).map(|i| Some(i)).chain_err(|| format!("Couldn't parse tag `resetMask` of register: `{}`", name)))?,
            fields: fields,
            write_constraint: tree.get_child("writeConstraint")
                    .map_or(
                        Ok(None), |constraint| {
                            WriteConstraint::parse(constraint)
                                .map(|c| Some(c))
                                .chain_err(|| "Couldn't parse writeConstraint")
                        }
                    )?,
            _extensible: (),
        })
    }
}

impl RegisterArrayInfo {
    fn parse(tree: &Element) -> Result<RegisterArrayInfo> {
        Ok(RegisterArrayInfo {
            dim: bail_if_none!(tree.get_child_text_try("dim")?, "Failed to get number of elements in dim").parse::<u32>().chain_err(|| "Couldn't parse dim")?,
            dim_increment: bail_if_none!(tree.get_child("dimIncrement").map_or(Ok(None), |t| parse::u32(t).map(|di| Some(di)).chain_err(|| "Couldn't parse dimIncrement"))?, "Couldn't get dimIncrement"),
            dim_index: tree.get_child("dimIndex").map_or(
                Ok(None),
                |c| parse::dim_index(bail_if_none!(c.text.as_ref(), "Couldn't get dimIndex"))
                .map(|di| Some(di)).chain_err(|| "Couldn't parse dimIndex"))?,
        })
    }
}

impl Register {
    // TODO handle "clusters", return `Register` not an `Option`
    fn parse(tree: &Element) -> Result<Option<Register>> {
        if tree.name == "cluster" {
            return Ok(None);
        }

        if tree.name != "register" {
            bail!("register not register")
        }
        let info = RegisterInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterArrayInfo::parse(tree)?;
            assert!(info.name.contains("%s"));
            if !info.name.contains("%s") {
                bail!("Register has dimIncrement, but contains no %s")
            }

            if let Some(ref indices) = array_info.dim_index {
                if array_info.dim as usize != indices.len() {
                    bail!("index doesn't match actual len"); // TODO: Fix error message
                }

            }
            Ok(Some(Register::Array(info, array_info)))
        } else {
            Ok(Some(Register::Single(info)))
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
    fn parse(tree: &Element) -> Result<Access> {
        let text = bail_if_none!(tree.text.as_ref(), "Couldn't get access");

        match &text[..] {
            "read-only" => Ok(Access::ReadOnly),
            "read-write" => Ok(Access::ReadWrite),
            "read-writeOnce" => Ok(Access::ReadWriteOnce),
            "write-only" => Ok(Access::WriteOnly),
            "writeOnce" => Ok(Access::WriteOnce),
            _ => bail!("unknown access variant: {}", text),
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
    fn parse(tree: &Element) -> Result<Field> {
        if tree.name != "field" {
            bail!("field not \"field\"")
        }

        Ok(
            Field {
                name: bail_if_none!(tree.get_child_text_try("name")?, "Couldn't get name"),
                description: tree.get_child_text_try("description")?,
                bit_range: BitRange::parse(tree)?,
                access: tree.get_child("access")
                    .map_or(
                        Ok(None), |access| {
                            Access::parse(access)
                                .map(|ac| Some(ac))
                                .chain_err(|| "Couldn't parse access")
                        }
                    )?,

                enumerated_values: {
                    let enumerated_values_inter =
                        tree.children
                            .iter()
                            .filter(|t| t.name == "enumeratedValues")
                            .map(EnumeratedValues::parse);
                    let mut enumerated_values = vec![];
                    for enumerated_values_res in enumerated_values_inter {
                        enumerated_values.push(enumerated_values_res?);
                    }
                    enumerated_values
                },

                write_constraint: tree.get_child("writeConstraint")
                    .map_or(
                        Ok(None), |constraint| {
                            WriteConstraint::parse(constraint)
                                .map(|c| Some(c))
                                .chain_err(|| "Couldn't parse writeConstraint")
                        }
                    )?,

                _extensible: (),
            },
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BitRange {
    pub offset: u32,
    pub width: u32,
}

impl BitRange {
    fn parse(tree: &Element) -> Result<BitRange> {
        let (end, start): (u32, u32) = if let Some(range) =
            tree.get_child("bitRange") {
            let text = bail_if_none!(range.text.as_ref(), "Couldn't get BitRange"); //TODO: Fix error message

            if !text.starts_with("[") {
                bail!("Invalid bitRange, expected `[` as first character");
            }
            if !text.ends_with("]") {
                bail!("Invalid bitRange, expected `]` as last character");
            }
            let mut parts = text[1..text.len() - 1].split(':');

            (bail_if_none!(
                bail_if_none!(parts.next(), "Invalid range").parse().ok(),
                "Failed to parse bitRange",
            ),
             bail_if_none!(
                bail_if_none!(parts.next(), "Invalid range").parse().ok(),
                "Failed to parse bitRange",
            ))
        } else if let (Some(lsb), Some(msb)) =
            (tree.get_child_text_try("lsb").ok().unwrap_or(None),
             tree.get_child_text_try("msb").ok().unwrap_or(None)) {
            (bail_if_none!(msb.parse().ok(), "Failed to parse msb"),
             bail_if_none!(lsb.parse::<u32>().ok(), "Failed to parse lsb"))
        } else {
            return Ok(BitRange {
                offset: bail_if_none!(
                            tree.get_child_text_try("bitOffset")?
                                .map_or(
                                    Ok(None),
                                    |offs| offs.parse().map(|int| Some(int))
                                        .chain_err(|| "Couldn't parse bitOffset")
                            )?,
                            "Couldn't get bitOffset"),
                width: bail_if_none!(
                            tree.get_child_text_try("bitWidth")?
                                .map_or(
                                    Ok(None),
                                    |offs| offs.parse().map(|int| Some(int))
                                        .chain_err(|| "Couldn't parse bitWidth")
                                )?,
                            "Couldn't get bitWidth"),
            });
        };

        Ok(
            BitRange {
                offset: start,
                width: end - start + 1,
            },
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WriteConstraintRange {
    pub min: u32,
    pub max: u32,
}

impl WriteConstraintRange {
    fn parse(tree: &Element) -> Result<WriteConstraintRange> {
        Ok(
            WriteConstraintRange {
                min: bail_if_none!(
                    tree.get_child_text_try("minimum")?,
                    "Couldn't get minimum",
                ).parse()?,
                max: bail_if_none!(
                    tree.get_child_text_try("maximum")?,
                    "Couldn't get maximum",
                ).parse()?,
            },
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WriteConstraint {
    WriteAsRead(bool),
    UseEnumeratedValues(bool),
    Range(WriteConstraintRange),
}

impl WriteConstraint {
    fn parse(tree: &Element) -> Result<WriteConstraint> {
        if tree.children.len() == 1 {
            let ref field = tree.children[0].name;
            // Write constraint can only be one of the following
            Ok(
                match field.as_ref() {
                    "writeAsRead" => {
                        WriteConstraint::WriteAsRead(parse::bool(tree.get_child_try(field.as_ref())?)?)
                    }
                    "useEnumeratedValues" => {
                        WriteConstraint::UseEnumeratedValues(parse::bool(tree.get_child_try(field.as_ref())?)?)
                    }
                    "range" => WriteConstraint::Range(WriteConstraintRange::parse(tree.get_child_try(field.as_ref())?)?,),
                    v => bail!("unknown <writeConstraint> variant: {}", v),
                },
            )
        } else {
            bail!("found more than one <WriteConstraint> element")
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
    fn parse(tree: &Element) -> Result<Defaults> {
        Ok(
            Defaults {
                size: tree.get_child("size")
                    .map_or(
                        Ok(None), |t| {
                            parse::u32(t)
                                .map(|int| Some(int))
                                .chain_err(|| "Couldn't parse size")
                        }
                    )?,
                reset_value: tree.get_child("resetValue")
                    .map_or(
                        Ok(None), |t| {
                            parse::u32(t)
                                .map(|int| Some(int))
                                .chain_err(|| "Couldn't parse resetValue")
                        }
                    )?,
                reset_mask: tree.get_child("resetMask")
                    .map_or(
                        Ok(None), |t| {
                            parse::u32(t)
                                .map(|int| Some(int))
                                .chain_err(|| "Couldn't parse resetMask")
                        }
                    )?,
                access: tree.get_child("access")
                    .map_or(
                        Ok(None), |val| {
                            Access::parse(val)
                                .map(|u| Some(u))
                                .chain_err(|| "Couldn't parse access")
                        }
                    )?,
                _extensible: (),
            },
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    Read,
    Write,
    ReadWrite,
}

impl Usage {
    fn parse(tree: &Element) -> Result<Usage> {
        let text = bail_if_none!(tree.text.as_ref(), "Couldn't get usage"); // TODO: Fix error message

        match &text[..] {
            "read" => Ok(Usage::Read),
            "write" => Ok(Usage::Write),
            "read-write" => Ok(Usage::ReadWrite),
            _ => bail!("unknown usage variant: {}", text),
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
    fn parse(tree: &Element) -> Result<EnumeratedValues> {
        if tree.name != "enumeratedValues" {
            bail!("enumeratedValues not enumeratedValues");
        }
        // If child is Ok(Some(_)) then use, otherwise don't, err on any Err(_)
        let values_inter = tree.children.iter().map(EnumeratedValue::parse);
        let mut values: Vec<EnumeratedValue> = vec![];
        for val_res in values_inter {
            if let Some(val) = val_res? {
                values.push(val);
            }
        }
        Ok(
            EnumeratedValues {
                name: tree.get_child_text_try("name").ok().unwrap_or(None),
                usage: tree.get_child("usage")
                    .map_or(
                        Ok(None), |usage| {
                            Usage::parse(usage)
                                .map(|u| Some(u))
                                .chain_err(|| "Couldn't parse usage")
                        }
                    )?,
                derived_from: tree.attributes
                    .get(&"derivedFrom".to_owned())
                    .map(|s| s.to_owned()),
                values: values,
                _extensible: (),
            },
        )
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
    fn parse(tree: &Element) -> Result<Option<EnumeratedValue>> {
        if tree.name != "enumeratedValue" {
            return Ok(None);
        }


        Ok(
            Some(
                EnumeratedValue {
                    name: bail_if_none!(
                        tree.get_child_text_try("name")?,
                        "Unable to get name",
                    ),
                    description: tree.get_child_text_try("description")?,
                    // Not the prettiest solution, but it works, probably can be hugely simplified
                    value: tree.get_child("value")
                        .map_or(
                            Ok(None), |elem| {
                                parse::u32(elem)
                                    .map(|int| Some(int))
                                    .chain_err(|| "Couldn't parse value")
                            }
                        )?,
                    is_default: tree.get_child_text_try("isDefault")?
                        .map_or(
                            Ok(None), |t| {
                                t.parse()
                                    .map(|t| Some(t))
                                    .chain_err(|| "Couldn't parse isDefault")
                            }
                        )?,
                    _extensible: (),
                },
            ),
        )
    }
}
