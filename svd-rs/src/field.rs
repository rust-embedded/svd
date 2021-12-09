use super::{array::SingleArray, FieldInfo};

/// Describes a field or fields of a [register](crate::RegisterInfo).
pub type Field = SingleArray<FieldInfo>;

impl Field {
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
