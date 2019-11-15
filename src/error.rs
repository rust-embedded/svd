//! SVD Errors.
//! This module defines error types and messages for SVD parsing and encoding

pub use anyhow::{anyhow, Context, Result};
use thiserror::Error;
use xmltree::Element;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Expected content in <{1}> tag, found none")]
    EmptyTag(Element, String),
    #[error("Expected a <{1}> tag, found none")]
    MissingTag(Element, String),
    #[error("NameMismatch")]
    NameMismatch(Element),
    #[error("Expected a <{1}>, found ...")]
    NotExpectedTag(Element, String),
    #[error("The content of the element could not be parsed to a boolean value {1}: {2}")]
    InvalidBooleanValue(Element, String, core::str::ParseBoolError),
    #[error("ParseError")]
    Other(Element),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum BuildError {
    #[error("`{0}` must be initialized")]
    Uninitialized(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum AccessTypeError {
    #[error("unknown access variant '{1}' found")]
    Unknown(Element, String),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum BitRangeError {
    #[error("Bit range invalid, {1:?}")]
    Invalid(Element, InvalidBitRange),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidBitRange {
    Empty,
    Syntax,
    ParseError,
    MsbLsb,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum EndianError {
    #[error("Unknown endianness `{0}`")]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ModifiedWriteValuesError {
    #[error("Invalid modifiedWriteValues variant, found {1}")]
    Invalid(Element, String),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DeviceError {
    #[error("Device must contain at least one peripheral")]
    Empty,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum PeripheralError {
    #[error("Peripheral have `registers` tag, but it is empty")]
    EmptyRegisters,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ClusterError {
    #[error("Cluster must contain at least one Register or Cluster")]
    Empty,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum RegisterError {
    #[error("Register have `fields` tag, but it is empty")]
    EmptyFields,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum RegisterClusterError {
    #[error("Invalid RegisterCluster (expected register or cluster), found {1}")]
    Invalid(Element, String),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum UsageVariantError {
    #[error("Unknown usage variant")]
    Unknown(Element),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum WriteConstraintError {
    #[error("Unknown write constraint")]
    Unknown(Element),
    #[error("Multiple wc found")]
    MoreThanOne(Element),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum NameError {
    #[error("Name `{0}` in tag `{1}` contains unexpected symbol")]
    Invalid(String, String),
}

pub(crate) fn is_valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    if let Some(first) = chars.next() {
        if !(first.is_ascii_alphabetic() || first == '_') {
            return false;
        }
        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '_' || c == '%') {
                return false;
            }
        }
        true
    } else {
        false
    }
}

pub(crate) fn check_name(name: &str, tag: &str) -> Result<()> {
    if is_valid_name(name) {
        Ok(())
    } else {
        Err(NameError::Invalid(name.to_string(), tag.to_string()).into())
    }
}
