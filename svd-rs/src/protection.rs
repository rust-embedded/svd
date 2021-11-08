/// Specify the security privilege to access an address region
///
/// This information is relevant for the programmer as well as the debugger
/// when no universal access permissions have been granted.
/// If no specific information is provided, an address region is accessible in any mode
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Protection {
    /// Secure permission required for access
    #[cfg_attr(feature = "serde", serde(rename = "s"))]
    Secure,

    /// Non-secure or secure permission required for access
    #[cfg_attr(feature = "serde", serde(rename = "n"))]
    NonSecure,

    /// Privileged permission required for access
    #[cfg_attr(feature = "serde", serde(rename = "p"))]
    Privileged,
}

impl Default for Protection {
    fn default() -> Self {
        Self::NonSecure
    }
}

impl Protection {
    /// Parse a string into an [`Protection`] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "s" => Some(Self::Secure),
            "n" => Some(Self::NonSecure),
            "p" => Some(Self::Privileged),
            _ => None,
        }
    }

    /// Convert this [`Protection`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Secure => "s",
            Self::NonSecure => "n",
            Self::Privileged => "p",
        }
    }
}
