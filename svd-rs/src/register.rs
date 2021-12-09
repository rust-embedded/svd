#[doc(hidden)]
pub use super::registercluster::{AllRegistersIter as RegIter, AllRegistersIterMut as RegIterMut};
use super::{array::SingleArray, RegisterInfo};

/// A single register or array of registers. A register is a named, programmable resource that belongs to a [peripheral](crate::Peripheral).
pub type Register = SingleArray<RegisterInfo>;

impl Register {
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
