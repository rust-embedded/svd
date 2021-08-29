#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
    Selectable,
    Other,
}

impl Default for Endian {
    fn default() -> Self {
        Self::Little
    }
}

impl Endian {
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "little" => Some(Self::Little),
            "big" => Some(Self::Big),
            "selectable" => Some(Self::Selectable),
            "other" => Some(Self::Other),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Little => "little",
            Self::Big => "big",
            Self::Selectable => "selectable",
            Self::Other => "other",
        }
    }
}
