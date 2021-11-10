use super::*;
use crate::svd::{BitRange, BitRangeType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidBitRange {
    Syntax,
    ParseError,
    MsbLsb,
    Empty,
    Size,
}

impl Parse for BitRange {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self, Self::Error> {
        let (end, start, range_type): (u32, u32, BitRangeType) =
            if let Some(range) = tree.get_child("bitRange") {
                let text = range.text().ok_or_else(|| {
                    SVDError::InvalidBitRange(InvalidBitRange::Empty).at(tree.id())
                })?;
                if !text.starts_with('[') {
                    return Err(SVDError::InvalidBitRange(InvalidBitRange::Syntax).at(tree.id()));
                    // TODO: Maybe have a MissingOpen/MissingClosing variant
                }
                if !text.ends_with(']') {
                    return Err(SVDError::InvalidBitRange(InvalidBitRange::Syntax).at(tree.id()));
                    // TODO: Maybe have a MissingOpen/MissingClosing variant
                }

                let mut parts = text[1..text.len() - 1].split(':');
                (
                    parts
                        .next()
                        .ok_or_else(|| {
                            SVDError::InvalidBitRange(InvalidBitRange::Syntax).at(tree.id())
                        })?
                        .parse::<u32>()
                        .map_err(|_| {
                            SVDError::InvalidBitRange(InvalidBitRange::ParseError).at(tree.id())
                        })?,
                    parts
                        .next()
                        .ok_or_else(|| {
                            SVDError::InvalidBitRange(InvalidBitRange::Syntax).at(tree.id())
                        })?
                        .parse::<u32>()
                        .map_err(|_| {
                            SVDError::InvalidBitRange(InvalidBitRange::ParseError).at(tree.id())
                        })?,
                    BitRangeType::BitRange,
                )
            // TODO: Consider matching instead so we can say which of these tags are missing
            } else if let (Some(lsb), Some(msb)) = (tree.get_child("lsb"), tree.get_child("msb")) {
                (
                    // TODO: `u32::parse` should not hide it's errors
                    u32::parse(&msb, &()).map_err(|_| {
                        SVDError::InvalidBitRange(InvalidBitRange::MsbLsb).at(tree.id())
                    })?,
                    u32::parse(&lsb, &()).map_err(|_| {
                        SVDError::InvalidBitRange(InvalidBitRange::MsbLsb).at(tree.id())
                    })?,
                    BitRangeType::MsbLsb,
                )
            } else if let (Some(offset), Some(width)) =
                (tree.get_child("bitOffset"), tree.get_child("bitWidth"))
            {
                // Special case because offset and width are directly provided
                // (ie. do not need to be calculated as in the final step)
                return Ok(BitRange {
                    // TODO: capture that error comes from offset/width tag
                    // TODO: `u32::parse` should not hide it's errors
                    offset: u32::parse(&offset, &()).map_err(|_| {
                        SVDError::InvalidBitRange(InvalidBitRange::ParseError).at(tree.id())
                    })?,
                    width: u32::parse(&width, &()).map_err(|_| {
                        SVDError::InvalidBitRange(InvalidBitRange::ParseError).at(tree.id())
                    })?,
                    range_type: BitRangeType::OffsetWidth,
                });
            } else {
                return Err(SVDError::InvalidBitRange(InvalidBitRange::Syntax).at(tree.id()));
            };

        if start > end {
            return Err(SVDError::InvalidBitRange(InvalidBitRange::Size).at(tree.id()));
        }
        Ok(Self {
            offset: start,
            width: end - start + 1,
            range_type,
        })
    }
}
