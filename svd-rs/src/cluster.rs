use core::ops::{Deref, DerefMut};
use std::borrow::Cow;

use super::{ClusterInfo, DimElement};

/// Cluster describes a sequence of neighboring registers within a peripheral.
#[derive(Clone, Debug, PartialEq)]
pub enum Cluster {
    /// A single cluster, without any dimension.
    Single(ClusterInfo),
    /// A cluster array
    Array(ClusterInfo, DimElement),
}

impl Deref for Cluster {
    type Target = ClusterInfo;

    fn deref(&self) -> &ClusterInfo {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl DerefMut for Cluster {
    fn deref_mut(&mut self) -> &mut ClusterInfo {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl Cluster {
    /// Return `true` if cluster instance is single
    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }
    /// Return `true` if it is cluster array
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
    /// Returns list of register or register array address_offsets
    pub fn address_offsets(&self) -> Vec<u32> {
        match self {
            Self::Single(info) => vec![info.address_offset],
            Self::Array(info, dim) => (0..dim.dim)
                .map(|n| info.address_offset + n * dim.dim_increment)
                .collect(),
        }
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use crate::{DeserArray, SerArray};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Cluster {
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

    impl<'de> Deserialize<'de> for Cluster {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let DeserArray { dim, info } = DeserArray::<ClusterInfo>::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(info.array(dim))
            } else {
                Ok(info.single())
            }
        }
    }
}
