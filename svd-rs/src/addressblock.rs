#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct AddressBlock {
    pub offset: u32,
    pub size: u32,
    pub usage: AddressBlockUsage,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressBlockUsage {
    Registers,
    Buffer,
    Reserved,
}

impl Default for AddressBlockUsage {
    fn default() -> Self {
        Self::Registers
    }
}

impl AddressBlockUsage {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "registers" => Some(Self::Registers),
            "buffer" => Some(Self::Buffer),
            "reserved" => Some(Self::Reserved),
            _ => None,
        }
    }

    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Registers => "registers",
            Self::Buffer => "wribufferte",
            Self::Reserved => "reserved",
        }
    }
}
