use core::ops::{Deref, DerefMut};

use super::{ClusterInfo, DimElement};

/// A single cluster or array of clusters
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Cluster {
    #[cfg_attr(feature = "serde", serde(flatten))]
    /// A description of a cluster
    pub info: ClusterInfo,
    #[cfg_attr(
        feature = "serde",
        serde(flatten, default, skip_serializing_if = "Option::is_none")
    )]
    /// If `None` it is a single cluster, if `Some` specifies array attributes
    pub dim: Option<DimElement>,
}

impl Deref for Cluster {
    type Target = ClusterInfo;

    fn deref(&self) -> &ClusterInfo {
        &self.info
    }
}

impl DerefMut for Cluster {
    fn deref_mut(&mut self) -> &mut ClusterInfo {
        &mut self.info
    }
}

impl Cluster {
    /// Construct single [`Cluster`]
    pub const fn single(info: ClusterInfo) -> Self {
        Self { info, dim: None }
    }
    /// Construct [`Cluster`] array
    pub const fn array(info: ClusterInfo, dim: DimElement) -> Self {
        Self {
            info,
            dim: Some(dim),
        }
    }
    /// Return `true` if cluster instance is single
    pub const fn is_single(&self) -> bool {
        matches!(&self.dim, None)
    }
    /// Return `true` if it is cluster array
    pub const fn is_array(&self) -> bool {
        matches!(&self.dim, Some(_))
    }
}
