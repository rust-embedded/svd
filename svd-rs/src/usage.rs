/// Allows specifying two different enumerated values
/// depending whether it is to be used for a read or a write access.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "kebab-case")
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    /// Read
    Read,
    /// Write
    Write,
    /// Read & Write
    ReadWrite,
}

impl Default for Usage {
    fn default() -> Self {
        Self::ReadWrite
    }
}

impl Usage {
    /// Parse a string into an [`Usage`] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "read" => Some(Self::Read),
            "write" => Some(Self::Write),
            "read-write" => Some(Self::ReadWrite),
            _ => None,
        }
    }

    /// Convert this [`Usage`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::ReadWrite => "read-write",
        }
    }
}
