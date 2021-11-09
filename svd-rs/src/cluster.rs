use core::ops::{Deref, DerefMut};

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

    /// Get the name of this cluster
    pub fn name(&self) -> &str {
        match self {
            Self::Single(info) => &info.name,
            Self::Array(info, _) => &info.name,
        }
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde::Deserialize, serde::Serialize)]
    struct ClusterArray {
        #[cfg_attr(
            feature = "serde",
            serde(flatten, default, skip_serializing_if = "Option::is_none")
        )]
        dim: Option<DimElement>,
        #[cfg_attr(feature = "serde", serde(flatten))]
        info: ClusterInfo,
    }

    impl Serialize for Cluster {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Single(info) => info.serialize(serializer),
                Self::Array(info, dim) => ClusterArray {
                    dim: Some(dim.clone()),
                    info: info.clone(),
                }
                .serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for Cluster {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let ClusterArray { dim, info } = ClusterArray::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(info.array(dim))
            } else {
                Ok(info.single())
            }
        }
    }
}
