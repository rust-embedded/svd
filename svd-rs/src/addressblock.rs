#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct AddressBlock {
    pub offset: u32,
    pub size: u32,
    pub usage: String,
}
