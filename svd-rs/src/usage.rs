/// Allows specifying two different enumerated values
/// depending whether it is to be used for a read or a write access.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    Read,
    Write,
    ReadWrite,
}

impl Default for Usage {
    fn default() -> Self {
        Self::ReadWrite
    }
}

impl Usage {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "read" => Some(Usage::Read),
            "write" => Some(Usage::Write),
            "read-write" => Some(Usage::ReadWrite),
            _ => None,
        }
    }
}
