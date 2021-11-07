use core::ops::{Deref, DerefMut};

use super::{DimElement, PeripheralInfo};

/// A single peripheral or array of peripherals
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Peripheral {
    #[cfg_attr(feature = "serde", serde(flatten))]
    /// A description of a peripheral
    pub info: PeripheralInfo,
    #[cfg_attr(
        feature = "serde",
        serde(flatten, default, skip_serializing_if = "Option::is_none")
    )]
    /// If `None` it is a single peripheral, if `Some` specifies array attributes
    pub dim: Option<DimElement>,
}

impl Deref for Peripheral {
    type Target = PeripheralInfo;

    fn deref(&self) -> &PeripheralInfo {
        &self.info
    }
}

impl DerefMut for Peripheral {
    fn deref_mut(&mut self) -> &mut PeripheralInfo {
        &mut self.info
    }
}

impl Peripheral {
    /// Construct single [`Peripheral`]
    pub const fn single(info: PeripheralInfo) -> Self {
        Self { info, dim: None }
    }
    /// Construct [`Peripheral`] array
    pub const fn array(info: PeripheralInfo, dim: DimElement) -> Self {
        Self {
            info,
            dim: Some(dim),
        }
    }
    /// Return `true` if peripheral instance is single
    pub const fn is_single(&self) -> bool {
        matches!(&self.dim, None)
    }
    /// Return `true` if it is peripheral array
    pub const fn is_array(&self) -> bool {
        matches!(&self.dim, Some(_))
    }
}
