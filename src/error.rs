//! SVD Errors.
//! This module defines error types and messages for SVD parsing and encoding

pub use anyhow::{Context, Result};
use xmltree::Element;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SVDError {
    #[error("Unknown endianness `{0}`")]
    UnknownEndian(String),
    // TODO: Needs context
    // TODO: Better name
    #[error("Expected a <{1}> tag, found none")]
    MissingTag(Element, String),
    #[error("Expected content in <{1}> tag, found none")]
    EmptyTag(Element, String),
    #[error("ParseError")]
    ParseError(Element),
    #[error("NameMismatch")]
    NameMismatch(Element),
    #[error("unknown access variant '{1}' found")]
    UnknownAccessType(Element, String),
    #[error("Bit range invalid, {1:?}")]
    InvalidBitRange(Element, InvalidBitRange),
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
    #[error("encoding method not implemented for svd object {0}")]
    EncodeNotImplemented(String),
    #[error("Error parsing SVD XML")]
    FileParseError,
}

// TODO: Consider making into an Error
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidBitRange {
    Syntax,
    ParseError,
    MsbLsb,
}
