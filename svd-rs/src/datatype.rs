/// Register data type
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataType {
    /// unsigned byte
    U8,
    /// unsigned half word
    U16,
    /// unsigned word
    U32,
    /// unsigned double word
    U64,
    /// signed byte
    I8,
    /// signed half word
    I16,
    /// signed world
    I32,
    /// signed double word
    I64,
    /// pointer to unsigned byte
    U8Ptr,
    /// pointer to unsigned half word
    U16Ptr,
    /// pointer to unsigned word
    U32Ptr,
    /// pointer to unsigned double word
    U64Ptr,
    /// pointer to signed byte
    I8Ptr,
    /// pointer to signed half word
    I16Ptr,
    /// pointer to signed world
    I32Ptr,
    /// pointer to signed double word
    I64Ptr,
}

impl DataType {
    /// Parse a string into an [`DataType`] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "uint8_t" => Some(Self::U8),
            "uint16_t" => Some(Self::U16),
            "uint32_t" => Some(Self::U32),
            "uint64_t" => Some(Self::U64),
            "int8_t" => Some(Self::I8),
            "int16_t" => Some(Self::I16),
            "int32_t" => Some(Self::I32),
            "int64_t" => Some(Self::I64),
            "uint8_t *" => Some(Self::U8Ptr),
            "uint16_t *" => Some(Self::U16Ptr),
            "uint32_t *" => Some(Self::U32Ptr),
            "uint64_t *" => Some(Self::U64Ptr),
            "int8_t *" => Some(Self::I8Ptr),
            "int16_t *" => Some(Self::I16Ptr),
            "int32_t *" => Some(Self::I32Ptr),
            "int64_t *" => Some(Self::I64Ptr),
            _ => None,
        }
    }

    /// Convert this [`DataType`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::U8 => "uint8_t",
            Self::U16 => "uint16_t",
            Self::U32 => "uint32_t",
            Self::U64 => "uint64_t",
            Self::I8 => "int8_t",
            Self::I16 => "int16_t",
            Self::I32 => "int32_t",
            Self::I64 => "int64_t",
            Self::U8Ptr => "uint8_t *",
            Self::U16Ptr => "uint16_t *",
            Self::U32Ptr => "uint32_t *",
            Self::U64Ptr => "uint64_t *",
            Self::I8Ptr => "int8_t *",
            Self::I16Ptr => "int16_t *",
            Self::I32Ptr => "int32_t *",
            Self::I64Ptr => "int64_t *",
        }
    }
}
