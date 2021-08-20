//! SVD objects.
//! This module defines components of an SVD along with parse and encode implementations

pub mod endian;
pub use self::endian::Endian;

pub mod cpu;
pub use self::cpu::Cpu;

pub mod interrupt;
pub use self::interrupt::Interrupt;

pub mod access;
pub use self::access::Access;

pub mod bitrange;
pub use self::bitrange::{BitRange, BitRangeType};

pub mod writeconstraint;
pub use self::writeconstraint::{WriteConstraint, WriteConstraintRange};

pub mod usage;
pub use self::usage::Usage;

pub mod enumeratedvalue;
pub use self::enumeratedvalue::EnumeratedValue;

pub mod enumeratedvalues;
pub use self::enumeratedvalues::EnumeratedValues;

pub mod field;
pub use self::field::Field;

pub mod fieldinfo;
pub use self::fieldinfo::FieldInfo;

pub mod registerinfo;
pub use self::registerinfo::RegisterInfo;

pub mod registerproperties;
pub use self::registerproperties::RegisterProperties;

pub mod addressblock;
pub use self::addressblock::{AddressBlock, AddressBlockUsage};

pub mod cluster;
pub use self::cluster::Cluster;

pub mod clusterinfo;
pub use self::clusterinfo::ClusterInfo;

pub mod register;
pub use self::register::Register;

pub mod registercluster;
pub use self::registercluster::RegisterCluster;

pub mod dimelement;
pub use self::dimelement::DimElement;

pub mod peripheral;
pub use self::peripheral::Peripheral;

pub mod device;
pub use self::device::Device;

pub mod modifiedwritevalues;
pub use self::modifiedwritevalues::ModifiedWriteValues;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValidateLevel {
    Disabled,
    Weak,
    Strict,
}

impl Default for ValidateLevel {
    fn default() -> Self {
        ValidateLevel::Weak
    }
}

impl ValidateLevel {
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

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SvdError {
    #[error("`Build error: {0}")]
    Build(#[from] BuildError),

    #[error("`Name check error: {0}")]
    Name(#[from] NameError),

    #[error("`Device error: {0}")]
    Device(#[from] device::Error),

    #[error("`Peripheral error: {0}")]
    Peripheral(#[from] peripheral::Error),

    #[error("`Cluster error: {0}")]
    Cluster(#[from] clusterinfo::Error),

    #[error("`Register error: {0}")]
    Register(#[from] registerinfo::Error),

    #[error("`Field error: {0}")]
    Field(#[from] fieldinfo::Error),

    #[error("`BitRange error: {0}")]
    BitRange(#[from] bitrange::Error),

    #[error("`EnumeratedValue error: {0}")]
    EnumeratedValue(#[from] enumeratedvalue::Error),

    #[error("`EnumeratedValues error: {0}")]
    EnumeratedValues(#[from] enumeratedvalues::Error),

    #[error("`RegisterProperties error: {0}")]
    RegisterProperties(#[from] registerproperties::Error),
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum BuildError {
    #[error("`{0}` must be initialized")]
    Uninitialized(String),
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NameError {
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
