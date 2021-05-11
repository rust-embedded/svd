use core::ops::{Deref, DerefMut};

use crate::svd::{clusterinfo::ClusterInfo, dimelement::DimElement};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
