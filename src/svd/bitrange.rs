#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("bitRange width of 0 does not make sense")]
    ZeroWidth,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitRange {
    /// Value defining the position of the least significant bit of the field within the register
    pub offset: u32,

    /// Value defining the bit-width of the bitfield within the register
    pub width: u32,

    pub range_type: BitRangeType,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BitRangeType {
    BitRange,
    OffsetWidth,
    MsbLsb,
}

impl BitRange {
    pub fn lsb(&self) -> u32 {
        self.offset
    }
    pub fn msb(&self) -> u32 {
        self.offset + self.width - 1
    }
}
