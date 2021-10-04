/// Errors for bit ranges
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The bit range is 0 bits wide
    #[error("bitRange width of 0 does not make sense")]
    ZeroWidth,
}

/// A bit range, describing the [least significant bit](Self::lsb) and [most significant bit](Self::msb)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitRange {
    /// Value defining the position of the least significant bit of the field within the register
    pub offset: u32,

    /// Value defining the bit-width of the bitfield within the register
    pub width: u32,

    /// The underlying description of the bit range
    pub range_type: BitRangeType,
}

/// The style of bit range that describes a [BitRange]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BitRangeType {
    /// A bit range in the format: `[<msb>:<lsb>]`
    BitRange,
    /// A bit range described as offset and width
    OffsetWidth,
    /// A bit range described as lsb and msb as separate elements
    MsbLsb,
}

impl BitRange {
    /// Get the position of the least significant bit
    pub fn lsb(&self) -> u32 {
        self.offset
    }
    /// Get the position of the most significant bit
    pub fn msb(&self) -> u32 {
        self.offset + self.width - 1
    }
    /// Get the bit range in the format `[<msb>:<lsb>]`
    pub fn bit_range(&self) -> String {
        format!("[{}:{}]", self.msb(), self.lsb())
    }
    /// Construct a [`BitRange`] from a offset and width
    pub fn from_offset_width(offset: u32, width: u32) -> Self {
        Self {
            offset,
            width,
            range_type: BitRangeType::OffsetWidth,
        }
    }

    /// Construct a [`BitRange`] from a msb and lsb
    pub fn from_msb_lsb(msb: u32, lsb: u32) -> Self {
        Self {
            offset: lsb,
            width: msb - lsb + 1,
            range_type: BitRangeType::MsbLsb,
        }
    }
    /// Construct a [`BitRange`] from a string in the format `[<msb>:<lsb>]`
    pub fn from_bit_range(text: &str) -> Option<Self> {
        if !text.starts_with('[') || !text.ends_with(']') {
            return None;
        }
        let mut parts = text[1..text.len() - 1].split(':');
        let msb = parts.next()?.parse::<u32>().ok()?;
        let lsb = parts.next()?.parse::<u32>().ok()?;
        Some(Self {
            offset: lsb,
            width: msb - lsb + 1,
            range_type: BitRangeType::BitRange,
        })
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::ser::SerializeMap;
    use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;

    impl Serialize for BitRange {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self.range_type {
                BitRangeType::BitRange => {
                    let mut seq = serializer.serialize_map(Some(4))?;
                    seq.serialize_entry("bitRange", &self.bit_range())?;
                    seq.end()
                }
                BitRangeType::OffsetWidth => {
                    let mut seq = serializer.serialize_map(Some(2))?;
                    seq.serialize_entry("bitOffset", &self.offset)?;
                    seq.serialize_entry("bitWidth", &self.width)?;
                    seq.end()
                }
                BitRangeType::MsbLsb => {
                    let mut seq = serializer.serialize_map(Some(2))?;
                    seq.serialize_entry("lsb", &self.lsb())?;
                    seq.serialize_entry("msb", &self.msb())?;
                    seq.end()
                }
            }
        }
    }

    impl<'de> Deserialize<'de> for BitRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(CustomVisitor)
        }
    }

    struct CustomVisitor;

    impl<'de> Visitor<'de> for CustomVisitor {
        type Value = BitRange;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "a map with keys 'bitRange' or 'bitOffset' and 'bitWidth' or 'lsb' and 'msb'"
            )
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            match map.next_key::<&str>()? {
                Some(k) if k == "bitRange" => {
                    let s: String = map.next_value()?;
                    BitRange::from_bit_range(&s)
                        .ok_or_else(|| serde::de::Error::custom("Can't parse bitRange"))
                }
                Some(k) if k == "bitOffset" || k == "bitWidth" => {
                    let offset;
                    let width;
                    if k == "bitOffset" {
                        offset = map.next_value()?;
                        width = match map.next_key::<&str>()? {
                            Some(k) if k == "bitWidth" => map.next_value()?,
                            _ => return Err(serde::de::Error::custom("Missing bitWidth")),
                        };
                    } else {
                        width = map.next_value()?;
                        offset = match map.next_key::<&str>()? {
                            Some(k) if k == "bitOffset" => map.next_value()?,
                            _ => return Err(serde::de::Error::custom("Missing bitOffset")),
                        };
                    }
                    Ok(BitRange::from_offset_width(offset, width))
                }
                Some(k) if k == "lsb" || k == "msb" => {
                    let msb;
                    let lsb;
                    if k == "msb" {
                        msb = map.next_value()?;
                        lsb = match map.next_key::<&str>()? {
                            Some(k) if k == "lsb" => map.next_value()?,
                            _ => return Err(serde::de::Error::custom("Missing lsb")),
                        };
                    } else {
                        lsb = map.next_value()?;
                        msb = match map.next_key::<&str>()? {
                            Some(k) if k == "msb" => map.next_value()?,
                            _ => return Err(serde::de::Error::custom("Missing msb")),
                        };
                    }
                    Ok(BitRange::from_msb_lsb(msb, lsb))
                }
                Some(k) => Err(serde::de::Error::custom(format!("Invalid key: {}", k))),
                None => Err(serde::de::Error::custom("Missing bitRange")),
            }
        }
    }
}
