//! SVD Errors.
//! This module defines error types and messages for SVD parsing and encoding

pub use anyhow::{Context, Result};
use core::u64;
use xmltree::Element;

#[allow(clippy::large_enum_variant, clippy::upper_case_acronyms)]
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
    Empty,
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

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ResetValueError {
    #[error("Reset value 0x{0:x} doesn't fit in {1} bits")]
    ValueTooLarge(u64, u32),
    #[error("Reset value 0x{0:x} conflicts with mask '0x{1:x}'")]
    MaskConflict(u64, u64),
    #[error("Mask value 0x{0:x} doesn't fit in {1} bits")]
    MaskTooLarge(u64, u32),
}

pub(crate) fn check_reset_value(
    size: Option<u32>,
    value: Option<u64>,
    _mask: Option<u64>,
) -> Result<()> {
    const MAX_BITS: u32 = u64::MAX.count_ones();

    if let (Some(size), Some(value)) = (size, value) {
        if MAX_BITS - value.leading_zeros() > size {
            return Err(ResetValueError::ValueTooLarge(value, size).into());
        }
    }
    #[cfg(feature = "strict")]
    {
        if let (Some(size), Some(mask)) = (size, _mask) {
            if MAX_BITS - mask.leading_zeros() > size {
                return Err(ResetValueError::MaskTooLarge(mask, size).into());
            }
        }
        if let (Some(value), Some(mask)) = (value, _mask) {
            if value & mask != value {
                return Err(ResetValueError::MaskConflict(value, mask).into());
            }
        }
    }

    Ok(())
}

#[cfg(feature = "strict")]
#[cfg(test)]
mod tests {
    use crate::error::check_reset_value;

    #[test]
    fn test_check_reset_value() {
        check_reset_value(None, None, None).unwrap();
        check_reset_value(Some(8), None, None).unwrap();
        check_reset_value(Some(8), None, Some(0xff)).unwrap();
        check_reset_value(Some(32), Some(0xfaceface), None).unwrap();
        check_reset_value(Some(32), Some(0xfaceface), Some(0xffffffff)).unwrap();

        assert!(
            check_reset_value(Some(8), None, Some(0x100)).is_err(),
            "mask shouldn't fit in size"
        );
        assert!(
            check_reset_value(Some(1), Some(0x02), None).is_err(),
            "reset value shouldn't fit in field"
        );
        assert!(
            check_reset_value(Some(8), Some(0x80), Some(0x01)).is_err(),
            "value should conflict with mask"
        );
    }
}
