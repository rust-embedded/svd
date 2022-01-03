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
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    #[allow(non_snake_case)]
    enum SerBitRange {
        BitRange { bitRange: String },
        OffsetWidth { bitOffset: u32, bitWidth: u32 },
        MsbLsb { lsb: u32, msb: u32 },
    }

    impl From<BitRange> for SerBitRange {
        fn from(br: BitRange) -> Self {
            match br.range_type {
                BitRangeType::BitRange => SerBitRange::BitRange {
                    bitRange: br.bit_range(),
                },
                BitRangeType::OffsetWidth => SerBitRange::OffsetWidth {
                    bitOffset: br.offset,
                    bitWidth: br.width,
                },
                BitRangeType::MsbLsb => SerBitRange::MsbLsb {
                    msb: br.msb(),
                    lsb: br.lsb(),
                },
            }
        }
    }

    impl Serialize for BitRange {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let bit_range = SerBitRange::from(*self);
            bit_range.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for BitRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            match SerBitRange::deserialize(deserializer)? {
                SerBitRange::BitRange { bitRange } => BitRange::from_bit_range(&bitRange)
                    .ok_or_else(|| serde::de::Error::custom("Can't parse bitRange")),
                SerBitRange::OffsetWidth {
                    bitOffset,
                    bitWidth,
                } => Ok(BitRange::from_offset_width(bitOffset, bitWidth)),
                SerBitRange::MsbLsb { msb, lsb } => Ok(BitRange::from_msb_lsb(msb, lsb)),
            }
        }
    }
}
