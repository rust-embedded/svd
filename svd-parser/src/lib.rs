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

use svd_rs as svd;

use xmltree::Element;
// ElementExt extends XML elements with useful methods
pub mod elementext;
// Types defines simple types and parse/encode implementations
pub mod types;

/// Parse trait allows SVD objects to be parsed from XML elements.
pub trait Parse {
    /// Object returned by parse method
    type Object;
    /// Parsing error
    type Error;
    /// Parse an XML/SVD element into it's corresponding `Object`.
    fn parse(elem: &Element) -> Result<Self::Object, Self::Error>;
}

/// Parses an optional child element with the provided name and Parse function
/// Returns an none if the child doesn't exist, Ok(Some(e)) if parsing succeeds,
/// and Err() if parsing fails.
pub fn optional<T>(n: &str, e: &Element) -> anyhow::Result<Option<T::Object>>
where
    T: Parse<Error = anyhow::Error>,
{
    let child = match e.get_child(n) {
        Some(c) => c,
        None => return Ok(None),
    };

    match T::parse(child) {
        Ok(r) => Ok(Some(r)),
        Err(e) => Err(e),
    }
}

use crate::svd::Device;
/// Parses the contents of an SVD (XML) string
pub fn parse(xml: &str) -> anyhow::Result<Device> {
    let xml = trim_utf8_bom(xml);
    let tree = Element::parse(xml.as_bytes())?;
    Device::parse(&tree)
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
mod register;
mod registercluster;
mod registerinfo;
mod registerproperties;
mod usage;
mod writeconstraint;

pub use anyhow::{Context, Result};

/// SVD parse Errors.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SVDError {
    #[error("Expected a <{1}> tag, found none")]
    MissingTag(Element, String),
    #[error("Expected content in <{1}> tag, found none")]
    EmptyTag(Element, String),
    #[error("ParseError")]
    ParseError(Element),
    #[error("NameMismatch")]
    NameMismatch(Element),
    #[error("Unknown endianness `{0}`")]
    UnknownEndian(String),
    #[error("unknown access variant '{1}' found")]
    UnknownAccessType(Element, String),
    #[error("Bit range invalid, {1:?}")]
    InvalidBitRange(Element, bitrange::InvalidBitRange),
    #[error("Unknown write constraint")]
    UnknownWriteConstraint(Element),
    #[error("Multiple wc found")]
    MoreThanOneWriteConstraint(Element),
    #[error("Unknown usage variant")]
    UnknownUsageVariant(Element),
    #[error("Expected a <{1}>, found ...")]
    NotExpectedTag(Element, String),
    #[error("Invalid RegisterCluster (expected register or cluster), found {1}")]
    InvalidRegisterCluster(Element, String),
    #[error("Invalid modifiedWriteValues variant, found {1}")]
    InvalidModifiedWriteValues(Element, String),
    #[error("The content of the element could not be parsed to a boolean value {1}: {2}")]
    InvalidBooleanValue(Element, String, core::str::ParseBoolError),
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NameParseError {
    #[error("Name `{0}` in tag `{1}` is missing a %s placeholder")]
    MissingPlaceholder(String, String),
}

pub(crate) fn check_has_placeholder(name: &str, tag: &str) -> Result<()> {
    if name.contains("%s") {
        Ok(())
    } else {
        Err(NameParseError::MissingPlaceholder(name.to_string(), tag.to_string()).into())
    }
}

#[test]
fn test_trim_utf8_bom_from_str() {
    // UTF-8 BOM + "xyz"
    let bom_str = std::str::from_utf8(b"\xef\xbb\xbfxyz").unwrap();
    assert_eq!("xyz", trim_utf8_bom(bom_str));
    assert_eq!("xyz", trim_utf8_bom("xyz"));
}
