///  An uniquely mapped address block to a peripheral
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct AddressBlock {
    /// Specifies the start address of an address block relative to the peripheral [`baseAddress`](crate::Peripheral::base_address).
    pub offset: u32,
    /// Specifies the number of [`addressUnitBits`](crate::Device::address_unit_bits) being covered by this address block.
    pub size: u32,
    /// Usage of the address block.
    pub usage: AddressBlockUsage,
}

/// Usage of the address block.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "kebab-case")
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressBlockUsage {
    /// Registers
    Registers,
    /// Buffer / Memory
    Buffer,
    /// Reserved
    Reserved,
}

impl Default for AddressBlockUsage {
    fn default() -> Self {
        Self::Registers
    }
}

impl AddressBlockUsage {
    /// Parse a string into an [`AddressBlockUsage`] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "registers" => Some(Self::Registers),
            "buffer" => Some(Self::Buffer),
            "reserved" => Some(Self::Reserved),
            _ => None,
        }
    }

    /// Convert this [`AddressBlockUsage`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Registers => "registers",
            Self::Buffer => "buffer",
            Self::Reserved => "reserved",
        }
    }
}
