use super::{array::SingleArray, ClusterInfo};

/// Cluster describes a sequence of neighboring registers within a peripheral.
pub type Cluster = SingleArray<ClusterInfo>;

impl Cluster {
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
