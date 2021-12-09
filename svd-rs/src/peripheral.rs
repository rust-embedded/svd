use core::ops::{Deref, DerefMut};
use std::borrow::Cow;

use super::{DimElement, PeripheralInfo};

/// A single peripheral or array of peripherals
#[derive(Clone, Debug, PartialEq)]
pub enum Peripheral {
    /// A single peripheral.
    Single(PeripheralInfo),
    /// An array of peripherals.
    Array(PeripheralInfo, DimElement),
}

impl Deref for Peripheral {
    type Target = PeripheralInfo;

    fn deref(&self) -> &PeripheralInfo {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl DerefMut for Peripheral {
    fn deref_mut(&mut self) -> &mut PeripheralInfo {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl Peripheral {
    /// Return `true` if peripheral instance is single
    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }
    /// Return `true` if it is peripheral array
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }
    /// Returns list of register or register array names
    pub fn names(&self) -> Vec<Cow<str>> {
        match self {
            Self::Single(info) => vec![info.name.as_str().into()],
            Self::Array(info, dim) => dim
                .indexes()
                .map(|i| info.name.replace("[%s]", &i).replace("%s", &i).into())
                .collect(),
        }
    }
    /// Returns list of register or register array memory addresses
    pub fn base_addresses(&self) -> Vec<u64> {
        match self {
            Self::Single(info) => vec![info.base_address],
            Self::Array(info, dim) => (0..dim.dim)
                .map(|n| info.base_address + (n as u64) * (dim.dim_increment as u64))
                .collect(),
        }
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use crate::{DeserArray, SerArray};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Peripheral {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Single(info) => info.serialize(serializer),
                Self::Array(info, dim) => SerArray { dim, info }.serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for Peripheral {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let DeserArray { dim, info } = DeserArray::<PeripheralInfo>::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(info.array(dim))
            } else {
                Ok(info.single())
            }
        }
    }
}
