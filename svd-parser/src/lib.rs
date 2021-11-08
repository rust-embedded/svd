//! CMSIS-SVD file parser
//!
//! # Usage
//!
//! ``` no_run
//! use svd_parser as svd;
//!
//! use std::fs::File;
//! use std::io::Read;
//!
//! let xml = &mut String::new();
//! File::open("STM32F30x.svd").unwrap().read_to_string(xml);
//!
//! println!("{:?}", svd::parse(xml));
//! ```
//!
//! # References
//!
//! - [SVD Schema file](https://www.keil.com/pack/doc/CMSIS/SVD/html/schema_1_2_gr.html)
//! - [SVD file database](https://github.com/posborne/cmsis-svd/tree/master/data)
//! - [Sample SVD file](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_Example_pg.html)

#![deny(warnings)]

//! Parse traits.
//! These support parsing of SVD types from XML

pub use svd::ValidateLevel;
pub use svd_rs as svd;

pub use anyhow::Context;
use roxmltree::{Document, Node, NodeId};
// ElementExt extends XML elements with useful methods
pub mod elementext;
use crate::elementext::ElementExt;
// Types defines simple types and parse/encode implementations
pub mod types;

#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct Config {
    pub validate_level: ValidateLevel,
    //pub expand_arrays: bool,
    //pub expand_derived: bool,
}

impl Config {
    pub fn validate_level(mut self, lvl: ValidateLevel) -> Self {
        self.validate_level = lvl;
        self
    }
}

/// Parse trait allows SVD objects to be parsed from XML elements.
pub trait Parse {
    /// Object returned by parse method
    type Object;
    /// Parsing error
    type Error;
    /// Advanced parse options
    type Config;
    /// Parse an XML/SVD element into it's corresponding `Object`.
    fn parse(elem: &Node, config: &Self::Config) -> Result<Self::Object, Self::Error>;
}

/// Parses an optional child element with the provided name and Parse function
/// Returns an none if the child doesn't exist, Ok(Some(e)) if parsing succeeds,
/// and Err() if parsing fails.
pub fn optional<T>(n: &str, e: &Node, config: &T::Config) -> Result<Option<T::Object>, SVDErrorAt>
where
    T: Parse<Error = SVDErrorAt>,
{
    let child = match e.get_child(n) {
        Some(c) => c,
        None => return Ok(None),
    };

    match T::parse(&child, config) {
        Ok(r) => Ok(Some(r)),
        Err(e) => Err(e),
    }
}

use crate::svd::Device;
/// Parses the contents of an SVD (XML) string
pub fn parse(xml: &str) -> anyhow::Result<Device> {
    parse_with_config(xml, &Config::default())
}
/// Parses the contents of an SVD (XML) string
pub fn parse_with_config(xml: &str, config: &Config) -> anyhow::Result<Device> {
    fn get_name<'a>(node: &'a Node) -> Option<&'a str> {
        node.children()
            .find(|t| t.has_tag_name("name"))
            .and_then(|t| t.text())
    }

    let xml = trim_utf8_bom(xml);
    let tree = Document::parse(xml)?;
    let root = tree.root();
    let device = root
        .get_child("device")
        .ok_or_else(|| SVDError::MissingTag("device".to_string()).at(root.id()))?;
    match Device::parse(&device, config) {
        Ok(o) => Ok(o),
        Err(e) => {
            let id = e.id;
            let node = tree.get_node(id).unwrap();
            let pos = tree.text_pos_at(node.range().start);
            let tagname = node.tag_name().name();
            let mut res = Err(e.into());
            if tagname.is_empty() {
                res = res.with_context(|| format!("at {}", pos))
            } else if let Some(name) = get_name(&node) {
                res = res.with_context(|| format!("Parsing {} `{}` at {}", tagname, name, pos))
            } else {
                res = res.with_context(|| format!("Parsing unknown {} at {}", tagname, pos))
            }
            for parent in node.ancestors().skip(1) {
                if parent.id() == NodeId::new(0) {
                    break;
                }
                let tagname = parent.tag_name().name();
                match tagname {
                    "device" | "peripheral" | "register" | "field" | "enumeratedValue"
                    | "interrupt" => {
                        if let Some(name) = get_name(&parent) {
                            res = res.with_context(|| format!("In {} `{}`", tagname, name));
                        } else {
                            res = res.with_context(|| format!("In unknown {}", tagname));
                        }
                    }
                    _ => {}
                }
            }
            res
        }
    }
}

/// Return the &str trimmed UTF-8 BOM if the input &str contains the BOM.
fn trim_utf8_bom(s: &str) -> &str {
    if s.len() > 2 && s.as_bytes().starts_with(b"\xef\xbb\xbf") {
        &s[3..]
    } else {
        s
    }
}

mod access;
mod addressblock;
mod bitrange;
mod cluster;
mod clusterinfo;
mod cpu;
mod device;
mod dimelement;
mod endian;
mod enumeratedvalue;
mod enumeratedvalues;
mod field;
mod fieldinfo;
mod interrupt;
mod modifiedwritevalues;
mod peripheral;
mod peripheralinfo;
mod protection;
mod readaction;
mod register;
mod registercluster;
mod registerinfo;
mod registerproperties;
mod usage;
mod writeconstraint;

/// SVD parse Errors.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum SVDError {
    #[error("{0}")]
    Svd(#[from] svd::SvdError),
    #[error("Expected a <{0}> tag, found none")]
    MissingTag(String),
    #[error("Expected content in <{0}> tag, found none")]
    EmptyTag(String),
    #[error("Failed to parse `{0}`")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Unknown endianness `{0}`")]
    UnknownEndian(String),
    #[error("unknown access variant '{0}' found")]
    UnknownAccessType(String),
    #[error("Bit range invalid, {0:?}")]
    InvalidBitRange(bitrange::InvalidBitRange),
    #[error("Unknown write constraint")]
    UnknownWriteConstraint,
    #[error("Multiple wc found")]
    MoreThanOneWriteConstraint,
    #[error("Unknown usage variant")]
    UnknownUsageVariant,
    #[error("Unknown usage variant for addressBlock")]
    UnknownAddressBlockUsageVariant,
    #[error("Expected a <{0}>, found ...")]
    NotExpectedTag(String),
    #[error("Invalid RegisterCluster (expected register or cluster), found {0}")]
    InvalidRegisterCluster(String),
    #[error("Invalid modifiedWriteValues variant, found {0}")]
    InvalidModifiedWriteValues(String),
    #[error("Invalid readAction variant, found {0}")]
    InvalidReadAction(String),
    #[error("Invalid protection variant, found {0}")]
    InvalidProtection(String),
    #[error("The content of the element could not be parsed to a boolean value {0}: {1}")]
    InvalidBooleanValue(String, core::str::ParseBoolError),
    #[error("dimIndex tag must contain {0} indexes, found {1}")]
    IncorrectDimIndexesCount(usize, usize),
    #[error("Failed to parse dimIndex")]
    DimIndexParse,
    #[error("Name `{0}` in tag `{1}` is missing a %s placeholder")]
    MissingPlaceholder(String, String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SVDErrorAt {
    error: SVDError,
    id: NodeId,
}

impl std::fmt::Display for SVDErrorAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl std::error::Error for SVDErrorAt {}

impl SVDError {
    pub fn at(self, id: NodeId) -> SVDErrorAt {
        SVDErrorAt { error: self, id }
    }
}

pub(crate) fn check_has_placeholder(name: &str, tag: &str) -> Result<(), SVDError> {
    if name.contains("%s") {
        Ok(())
    } else {
        Err(SVDError::MissingPlaceholder(
            name.to_string(),
            tag.to_string(),
        ))
    }
}

#[test]
fn test_trim_utf8_bom_from_str() {
    // UTF-8 BOM + "xyz"
    let bom_str = std::str::from_utf8(b"\xef\xbb\xbfxyz").unwrap();
    assert_eq!("xyz", trim_utf8_bom(bom_str));
    assert_eq!("xyz", trim_utf8_bom("xyz"));
}
