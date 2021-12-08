#![deny(missing_docs)]
//! SVD objects.
//! This module defines components of an SVD along with parse and encode implementations

/// Endian objects
pub mod endian;
pub use self::endian::Endian;

/// Cpu objects
pub mod cpu;
pub use self::cpu::{Cpu, CpuBuilder};

/// Interrupt objects
pub mod interrupt;
pub use self::interrupt::Interrupt;

/// Access objects
pub mod access;
pub use self::access::Access;

/// Bitrange objects
pub mod bitrange;
pub use self::bitrange::{BitRange, BitRangeType};

/// Write constraint objects
pub mod writeconstraint;
pub use self::writeconstraint::{WriteConstraint, WriteConstraintRange};

/// Usage objects
pub mod usage;
pub use self::usage::Usage;

/// Enumerated Value objects
pub mod enumeratedvalue;
pub use self::enumeratedvalue::{EnumeratedValue, EnumeratedValueBuilder};

/// Enumerated Values objects
pub mod enumeratedvalues;
pub use self::enumeratedvalues::{EnumeratedValues, EnumeratedValuesBuilder};

/// Field objects
pub mod field;
pub use self::field::Field;

/// Field Info objects
pub mod fieldinfo;
pub use self::fieldinfo::{FieldInfo, FieldInfoBuilder};

/// Register Info objects
pub mod registerinfo;
pub use self::registerinfo::{RegisterInfo, RegisterInfoBuilder};

/// Register Properties objects
pub mod registerproperties;
pub use self::registerproperties::RegisterProperties;

/// Address Block objects
pub mod addressblock;
pub use self::addressblock::{AddressBlock, AddressBlockUsage};

/// Cluster objects
pub mod cluster;
pub use self::cluster::Cluster;

/// Cluster Info objects
pub mod clusterinfo;
pub use self::clusterinfo::{ClusterInfo, ClusterInfoBuilder};

/// Register objects
pub mod register;
pub use self::register::Register;

/// Register Cluster objects
pub mod registercluster;
pub use self::registercluster::RegisterCluster;

/// Dimelement objects
pub mod dimelement;
pub use self::dimelement::{DimArrayIndex, DimElement, DimElementBuilder};

/// Peripheral objects
pub mod peripheral;
pub use self::peripheral::Peripheral;

/// Peripheral Info objects
pub mod peripheralinfo;
pub use self::peripheralinfo::{PeripheralInfo, PeripheralInfoBuilder};

/// Device objects
pub mod device;
pub use self::device::{Device, DeviceBuilder};

/// Modified Write Values objects
pub mod modifiedwritevalues;
pub use self::modifiedwritevalues::ModifiedWriteValues;

/// Read Action objects
pub mod readaction;
pub use self::readaction::ReadAction;

/// Protection objects
pub mod protection;
pub use self::protection::Protection;

/// Level of validation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValidateLevel {
    /// No validation.
    Disabled,
    /// Weak validation.
    Weak,
    /// Strict validation.
    Strict,
}

impl Default for ValidateLevel {
    fn default() -> Self {
        ValidateLevel::Weak
    }
}

impl ValidateLevel {
    /// Returns true if validation is disabled.
    pub fn is_disabled(self) -> bool {
        self == ValidateLevel::Disabled
    }
    /// Returns true if validation is considered to be weakly checked.
    pub fn is_weak(self) -> bool {
        self != ValidateLevel::Disabled
    }
    /// Returns true if validation is considered to be strictly checked.
    pub fn is_strict(self) -> bool {
        self == ValidateLevel::Strict
    }
}

#[cfg(feature = "derive-from")]
pub mod derive_from;
#[cfg(feature = "derive-from")]
pub use derive_from::DeriveFrom;

use once_cell::sync::Lazy;
use regex::Regex;

/// Errors that can occur during building.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SvdError {
    /// Error related to a builder
    #[error("`Build error: {0}")]
    Build(#[from] BuildError),
    /// Name check error
    #[error("`Name check error: {0}")]
    Name(#[from] NameError),
    /// Device error
    #[error("`Device error: {0}")]
    Device(#[from] device::Error),
    /// Peripheral error
    #[error("`Peripheral error: {0}")]
    Peripheral(#[from] peripheralinfo::Error),
    /// Cluster error
    #[error("`Cluster error: {0}")]
    Cluster(#[from] clusterinfo::Error),
    /// Register error
    #[error("`Register error: {0}")]
    Register(#[from] registerinfo::Error),
    /// Field error
    #[error("`Field error: {0}")]
    Field(#[from] fieldinfo::Error),
    /// BitRange error
    #[error("`BitRange error: {0}")]
    BitRange(#[from] bitrange::Error),
    /// EnumeratedValue error
    #[error("`EnumeratedValue error: {0}")]
    EnumeratedValue(#[from] enumeratedvalue::Error),
    /// EnumeratedValues error
    #[error("`EnumeratedValues error: {0}")]
    EnumeratedValues(#[from] enumeratedvalues::Error),
    /// RegisterProperties error
    #[error("`RegisterProperties error: {0}")]
    RegisterProperties(#[from] registerproperties::Error),
}

/// Errors from a builder
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum BuildError {
    /// Field was not set when building it.
    #[error("`{0}` must be initialized")]
    Uninitialized(String),
}

/// Invalid error
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NameError {
    /// Name is invalid
    #[error("Name `{0}` contains unexpected symbol")]
    Invalid(String, String),
}

pub(crate) fn check_name(name: &str, tag: &str) -> Result<(), NameError> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^[_A-Za-z0-9]*$").unwrap());
    if PATTERN.is_match(name) {
        Ok(())
    } else {
        Err(NameError::Invalid(name.to_string(), tag.to_string()))
    }
}

pub(crate) fn check_dimable_name(name: &str, tag: &str) -> Result<(), NameError> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new("^(((%s)|(%s)[_A-Za-z]{1}[_A-Za-z0-9]*)|([_A-Za-z]{1}[_A-Za-z0-9]*(\\[%s\\])?)|([_A-Za-z]{1}[_A-Za-z0-9]*(%s)?[_A-Za-z0-9]*))$").unwrap()
    });
    if PATTERN.is_match(name) {
        Ok(())
    } else {
        Err(NameError::Invalid(name.to_string(), tag.to_string()))
    }
}

pub(crate) fn check_derived_name(name: &str, tag: &str) -> Result<(), NameError> {
    for x in name.split('.') {
        check_dimable_name(x, tag)?
    }
    Ok(())
}

trait EmptyToNone {
    fn empty_to_none(self) -> Self;
}

impl EmptyToNone for Option<String> {
    fn empty_to_none(self) -> Self {
        self.and_then(|s| if s.is_empty() { None } else { Some(s) })
    }
}

impl<T> EmptyToNone for Option<Vec<T>> {
    fn empty_to_none(self) -> Self {
        self.and_then(|v| if v.is_empty() { None } else { Some(v) })
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize)]
struct SerArray<'a, T> {
    #[serde(flatten)]
    dim: &'a DimElement,
    #[serde(flatten)]
    info: &'a T,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct DeserArray<T> {
    #[serde(flatten, default)]
    dim: Option<DimElement>,
    #[serde(flatten)]
    info: T,
}

/// Iterates over optional iterator
pub struct OptIter<I>(Option<I>)
where
    I: Iterator;

impl<I> OptIter<I>
where
    I: Iterator,
{
    /// Create new optional iterator
    pub fn new(o: Option<I>) -> Self {
        Self(o)
    }
}

impl<'a, I> Iterator for OptIter<I>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().and_then(I::next)
    }
}
