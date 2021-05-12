use core::ops::{Deref, DerefMut};

use crate::svd::{clusterinfo::ClusterInfo, dimelement::DimElement};

#[derive(Clone, Debug, PartialEq)]
pub enum Cluster {
    Single(ClusterInfo),
    Array(ClusterInfo, DimElement),
}

impl Deref for Cluster {
    type Target = ClusterInfo;

    fn deref(&self) -> &ClusterInfo {
        match self {
            Cluster::Single(info) => info,
            Cluster::Array(info, _) => info,
        }
    }
}

impl DerefMut for Cluster {
    fn deref_mut(&mut self) -> &mut ClusterInfo {
        match self {
            Cluster::Single(info) => info,
            Cluster::Array(info, _) => info,
        }
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde::Deserialize, serde::Serialize)]
    struct ClusterArray {
        #[cfg_attr(feature = "serde", serde(flatten))]
        #[cfg_attr(feature = "serde", serde(default))]
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
                Cluster::Single(info) => info.serialize(serializer),
                Cluster::Array(info, dim) => ClusterArray {
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
                Ok(Cluster::Array(info, dim))
            } else {
                Ok(Cluster::Single(info))
            }
        }
    }
}
