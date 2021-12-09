use super::{DimElement, FieldInfo};
use core::ops::{Deref, DerefMut};
use std::borrow::Cow;

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
    /// Returns list of register or register array bit offsets
    pub fn bit_offsets(&self) -> Vec<u32> {
        match self {
            Self::Single(info) => vec![info.bit_offset()],
            Self::Array(info, dim) => (0..dim.dim)
                .map(|n| info.bit_offset() + n * dim.dim_increment)
                .collect(),
        }
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use crate::{DeserArray, SerArray};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Field {
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

    impl<'de> Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let DeserArray { dim, info } = DeserArray::<FieldInfo>::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(info.array(dim))
            } else {
                Ok(info.single())
            }
        }
    }
}
