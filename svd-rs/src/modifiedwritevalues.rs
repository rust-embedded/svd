/// Describe the manipulation of data written to a register/field.
/// If not specified, the value written to the field is the value stored in the field
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModifiedWriteValues {
    /// Write data bit of one shall clear (set to zero) the corresponding bit in the field
    OneToClear,

    /// Write data bit of one shall set (set to one) the corresponding bit in the field
    OneToSet,

    /// Write data bit of one shall toggle (invert) the corresponding bit in the field
    OneToToggle,

    /// Write data bit of zero shall clear (set to zero) the corresponding bit in the field
    ZeroToClear,

    /// Write data bit of zero shall set (set to one) the corresponding bit in the field
    ZeroToSet,

    /// Write data bit of zero shall toggle (invert) the corresponding bit in the field
    ZeroToToggle,

    /// After a write operation all bits in the field are cleared (set to zero)
    Clear,

    /// After a write operation all bits in the field are set (set to one)
    Set,

    /// After a write operation all bit in the field may be modified (default)
    Modify,
}

impl Default for ModifiedWriteValues {
    fn default() -> Self {
        Self::Modify
    }
}

impl ModifiedWriteValues {
    /// Parse a string into an [`ModifiedWriteValues`] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        use self::ModifiedWriteValues::*;
        match s {
            "oneToClear" => Some(OneToClear),
            "oneToSet" => Some(OneToSet),
            "oneToToggle" => Some(OneToToggle),
            "zeroToClear" => Some(ZeroToClear),
            "zeroToSet" => Some(ZeroToSet),
            "zeroToToggle" => Some(ZeroToToggle),
            "clear" => Some(Clear),
            "set" => Some(Set),
            "modify" => Some(Modify),
            _ => None,
        }
    }

    /// Convert this [`ModifiedWriteValues`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneToClear => "oneToClear",
            Self::OneToSet => "oneToSet",
            Self::OneToToggle => "oneToToggle",
            Self::ZeroToClear => "zeroToClear",
            Self::ZeroToSet => "zeroToSet",
            Self::ZeroToToggle => "zeroToToggle",
            Self::Clear => "clear",
            Self::Set => "set",
            Self::Modify => "modify",
        }
    }
}
