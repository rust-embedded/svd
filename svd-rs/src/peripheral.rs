use core::ops::{Deref, DerefMut};

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
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde::Deserialize, serde::Serialize)]
    struct PeripheralArray {
        #[cfg_attr(
            feature = "serde",
            serde(flatten, default, skip_serializing_if = "Option::is_none")
        )]
        dim: Option<DimElement>,
        #[cfg_attr(feature = "serde", serde(flatten))]
        info: PeripheralInfo,
    }

    impl Serialize for Peripheral {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Single(info) => info.serialize(serializer),
                Self::Array(info, dim) => PeripheralArray {
                    dim: Some(dim.clone()),
                    info: info.clone(),
                }
                .serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for Peripheral {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let PeripheralArray { dim, info } = PeripheralArray::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(info.array(dim))
            } else {
                Ok(info.single())
            }
        }
    }
}
