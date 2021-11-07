use super::{DimElement, FieldInfo};
use core::ops::{Deref, DerefMut};

/// Describes a field or fields of a [register](crate::RegisterInfo).
#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    /// A single field.
    Single(FieldInfo),
    /// A field array.
    Array(FieldInfo, DimElement),
}

impl Deref for Field {
    type Target = FieldInfo;

    fn deref(&self) -> &FieldInfo {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut FieldInfo {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl Field {
    /// Return `true` if field instance is single
    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }
    /// Return `true` if it is field array
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde::Deserialize, serde::Serialize)]
    struct FieldArray {
        #[cfg_attr(
            feature = "serde",
            serde(flatten, default, skip_serializing_if = "Option::is_none")
        )]
        dim: Option<DimElement>,
        #[cfg_attr(feature = "serde", serde(flatten))]
        info: FieldInfo,
    }

    impl Serialize for Field {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Single(info) => info.serialize(serializer),
                Self::Array(info, dim) => FieldArray {
                    dim: Some(dim.clone()),
                    info: info.clone(),
                }
                .serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let FieldArray { dim, info } = FieldArray::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(info.array(dim))
            } else {
                Ok(info.single())
            }
        }
    }
}
