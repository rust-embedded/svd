use super::{array::SingleArray, PeripheralInfo};

/// A single peripheral or array of peripherals
pub type Peripheral = SingleArray<PeripheralInfo>;

impl Peripheral {
    /// Returns list of register or register array memory addresses
    pub fn base_addresses(&self) -> Vec<u64> {
        match self {
            Self::Single(info) => vec![info.base_address],
            Self::Array(info, dim) => (0..dim.dim)
                .map(|n| info.base_address + (n as u64) * (dim.dim_increment as u64))
                .collect(),
        }
    }
}
