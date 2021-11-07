/// Specifies the side effect following a read operation
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadAction {
    /// The register/field is cleared (set to zero) following a read operation
    Clear,

    /// The register/field is set (set to ones) following a read operation
    Set,

    /// The register/field is modified in some way after a read operation
    Modify,

    /// One or more dependent resources other than the current register/field are immediately affected by a read operation
    ModifyExternal,
}

impl Default for ReadAction {
    fn default() -> Self {
        Self::Modify
    }
}

impl ReadAction {
    /// Parse a string into an [`ReadAction`] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        use self::ReadAction::*;
        match s {
            "clear" => Some(Clear),
            "set" => Some(Set),
            "modify" => Some(Modify),
            "modifyExternal" => Some(ModifyExternal),
            _ => None,
        }
    }

    /// Convert this [`ReadAction`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Set => "set",
            Self::Modify => "modify",
            Self::ModifyExternal => "modifyExternal",
        }
    }
}
