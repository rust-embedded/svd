/// Endianness of a [processor](crate::Cpu).
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "kebab-case")
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    /// Little endian.
    Little,
    /// Big endian.
    Big,
    /// Mixed endian.
    Selectable,
    /// Other
    Other,
}

impl Default for Endian {
    fn default() -> Self {
        Self::Little
    }
}

impl Endian {
    /// Parse a string into an [Endian] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "little" => Some(Self::Little),
            "big" => Some(Self::Big),
            "selectable" => Some(Self::Selectable),
            "other" => Some(Self::Other),
            _ => None,
        }
    }

    /// Convert this [`Endian`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Little => "little",
            Self::Big => "big",
            Self::Selectable => "selectable",
            Self::Other => "other",
        }
    }
}
