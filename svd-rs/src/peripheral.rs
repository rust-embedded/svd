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
            Peripheral::Single(info) => info,
            Peripheral::Array(info, _) => info,
        }
    }
}

impl DerefMut for Peripheral {
    fn deref_mut(&mut self) -> &mut PeripheralInfo {
        match self {
            Peripheral::Single(info) => info,
            Peripheral::Array(info, _) => info,
        }
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
                Peripheral::Single(info) => info.serialize(serializer),
                Peripheral::Array(info, dim) => PeripheralArray {
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
                Ok(Peripheral::Array(info, dim))
            } else {
                Ok(Peripheral::Single(info))
            }
        }
    }
}
