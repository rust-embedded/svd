/// Defines access rights for fields on the device, though it may be specified at a
/// higher level than individual fields.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Access {
    /// Read access is permitted. Write operations have an undefined effect.
    #[cfg_attr(feature = "serde", serde(rename = "read-only"))]
    ReadOnly,

    /// Read and write accesses are permitted.
    #[cfg_attr(feature = "serde", serde(rename = "read-write"))]
    ReadWrite,

    /// Read access is always permitted.
    /// Only the first write after a reset will affect the content.
    /// Following writes have an undefined effect.
    #[cfg_attr(feature = "serde", serde(rename = "read-writeOnce"))]
    ReadWriteOnce,

    /// Read operations have undefined results.
    /// Only the first write after a reset will affect the content.
    #[cfg_attr(feature = "serde", serde(rename = "writeOnce"))]
    WriteOnce,

    /// Read operations have an undefined result. Write access is permitted.
    #[cfg_attr(feature = "serde", serde(rename = "write-only"))]
    WriteOnly,
}

impl Access {
    /// Whether the register/field is readable at least once.
    pub fn can_read(self) -> bool {
        matches!(self, Self::ReadOnly | Self::ReadWrite | Self::ReadWriteOnce)
    }

    /// Whether the register/field is writable at least once.
    pub fn can_write(self) -> bool {
        !matches!(self, Self::ReadOnly)
    }
}

impl Default for Access {
    fn default() -> Self {
        Self::ReadWrite
    }
}

impl Access {
    /// Parse a string into an [`Access`] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "read-only" => Some(Self::ReadOnly),
            "read-write" => Some(Self::ReadWrite),
            "read-writeOnce" => Some(Self::ReadWriteOnce),
            "write-only" => Some(Self::WriteOnly),
            "writeOnce" => Some(Self::WriteOnce),
            _ => None,
        }
    }

    /// Convert this [`Access`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::ReadWrite => "read-write",
            Self::ReadWriteOnce => "read-writeOnce",
            Self::WriteOnly => "write-only",
            Self::WriteOnce => "writeOnce",
        }
    }
}
