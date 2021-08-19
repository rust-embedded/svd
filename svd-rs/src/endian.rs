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
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "little" => Some(Endian::Little),
            "big" => Some(Endian::Big),
            "selectable" => Some(Endian::Selectable),
            "other" => Some(Endian::Other),
            _ => None,
        }
    }
}
