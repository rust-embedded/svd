#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Interrupt {
    /// The string represents the interrupt name
    pub name: String,

    /// The string describes the interrupt
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Represents the enumeration index value associated to the interrupt
    pub value: u32,
}
