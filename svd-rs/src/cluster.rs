use super::{array::SingleArray, ClusterInfo};

/// Cluster describes a sequence of neighboring registers within a peripheral.
pub type Cluster = SingleArray<ClusterInfo>;
