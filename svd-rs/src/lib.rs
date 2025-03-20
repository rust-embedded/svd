#![deny(missing_docs)]
//! SVD objects.
//! This module defines components of an SVD along with parse and encode implementations

/// Common things for structures which can be collected in arrays
pub mod array;
pub use array::MaybeArray;

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
pub use self::field::{Field, FieldInfo, FieldInfoBuilder};

/// Register Properties objects
pub mod registerproperties;
pub use self::registerproperties::RegisterProperties;

/// Address Block objects
pub mod addressblock;
pub use self::addressblock::{AddressBlock, AddressBlockUsage};

/// Cluster objects
pub mod cluster;
pub use self::cluster::{Cluster, ClusterInfo, ClusterInfoBuilder};

/// Register objects
pub mod register;
pub use self::register::{Register, RegisterInfo, RegisterInfoBuilder};

/// Register Cluster objects
pub mod registercluster;
pub use self::registercluster::RegisterCluster;

/// Dimelement objects
pub mod dimelement;
pub use self::dimelement::{DimArrayIndex, DimElement, DimElementBuilder};

/// Peripheral objects
pub mod peripheral;
pub use self::peripheral::{Peripheral, PeripheralInfo, PeripheralInfoBuilder};

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

/// DataType objects
pub mod datatype;
pub use self::datatype::DataType;

/// Level of validation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ValidateLevel {
    /// No validation.
    Disabled,
    /// Weak validation.
    #[default]
    Weak,
    /// Strict validation.
    Strict,
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

use fancy_regex::Regex;
use once_cell::sync::Lazy;

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
    Peripheral(#[from] peripheral::Error),
    /// Cluster error
    #[error("`Cluster error: {0}")]
    Cluster(#[from] cluster::Error),
    /// Register error
    #[error("`Register error: {0}")]
    Register(#[from] register::Error),
    /// Field error
    #[error("`Field error: {0}")]
    Field(#[from] field::Error),
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
    /// WriteConstraint error
    #[error("`WriteConstraint error: {0}")]
    WriteConstraint(#[from] writeconstraint::Error),
}

/// Errors from a builder
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum BuildError {
    /// Field was not set when building it.
    #[error("`{0}` must be initialized")]
    Uninitialized(String),
    /// Unbuildable
    #[error("Unbuildable")],
    Unbuildable,
}

/// Invalid error
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NameError {
    /// Name is invalid
    #[error("Name `{0}` contains unexpected symbol")]
    Invalid(String, String),
    /// Regex matching error
    #[error("Error matching to regex")]
    Regex,
}

pub(crate) fn check_name(name: &str, tag: &str) -> Result<(), NameError> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^[_A-Za-z0-9]*$").unwrap());
    if PATTERN.is_match(name).map_err(|_| NameError::Regex)? {
        Ok(())
    } else {
        Err(NameError::Invalid(name.to_string(), tag.to_string()))
    }
}

pub(crate) fn check_dimable_name(name: &str, tag: &str) -> Result<(), NameError> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new("^(((%s)|(%s)[_A-Za-z]{1}[_A-Za-z0-9]*)|([_A-Za-z]{1}[_A-Za-z0-9]*(\\[%s\\])?)|([_A-Za-z]{1}[_A-Za-z0-9]*(%s)?[_A-Za-z0-9]*))$").unwrap()
    });
    if PATTERN.is_match(name).map_err(|_| NameError::Regex)? {
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

/// Get SVD element name
pub trait Name {
    /// Get name
    fn name(&self) -> &str;
}

impl<T> Name for &T
where
    T: Name,
{
    fn name(&self) -> &str {
        T::name(*self)
    }
}

impl<T> Name for &mut T
where
    T: Name,
{
    fn name(&self) -> &str {
        T::name(*self)
    }
}

/// Get SVD element description
pub trait Description {
    /// Get description
    fn description(&self) -> Option<&str>;
}

impl<T> Description for &T
where
    T: Description,
{
    fn description(&self) -> Option<&str> {
        T::description(*self)
    }
}

impl<T> Description for &mut T
where
    T: Description,
{
    fn description(&self) -> Option<&str> {
        T::description(*self)
    }
}

#[derive(Clone, Debug)]
enum MString {
    String(String),
    Replace(fancy_regex::Regex, String),
}

impl Default for MString {
    fn default() -> Self {
        Self::String(String::default())
    }
}

impl From<String> for MString {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl MString {
    pub fn as_str(self) -> Result<String, BuildError> {
        match self {
            Self::String(s) => Ok(s),
            Self::Replace(_, _) => Err(BuildError::Unbuildable),
        }
    }
}
