#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Access {
    #[cfg_attr(feature = "serde", serde(rename = "read-only"))]
    ReadOnly,
    #[cfg_attr(feature = "serde", serde(rename = "read-write"))]
    ReadWrite,
    #[cfg_attr(feature = "serde", serde(rename = "read-writeOnce"))]
    ReadWriteOnce,
    #[cfg_attr(feature = "serde", serde(rename = "writeOnce"))]
    WriteOnce,
    #[cfg_attr(feature = "serde", serde(rename = "write-only"))]
    WriteOnly,
}
