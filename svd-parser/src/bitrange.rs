use super::{Context, Element, Parse, Result, SVDError};
use crate::elementext::ElementExt;
use crate::svd::{BitRange, BitRangeType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidBitRange {
    Syntax,
    ParseError,
    MsbLsb,
    Empty,
}

impl Parse for BitRange {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let (end, start, range_type): (u32, u32, BitRangeType) = if let Some(range) =
            tree.get_child("bitRange")
        {
            let text = range
                .text()
                .ok_or_else(|| SVDError::InvalidBitRange(tree.id(), InvalidBitRange::Empty))?;
            if !text.starts_with('[') {
                return Err(SVDError::InvalidBitRange(tree.id(), InvalidBitRange::Syntax).into());
                // TODO: Maybe have a MissingOpen/MissingClosing variant
            }
            if !text.ends_with(']') {
                return Err(SVDError::InvalidBitRange(tree.id(), InvalidBitRange::Syntax).into());
                // TODO: Maybe have a MissingOpen/MissingClosing variant
            }

            let mut parts = text[1..text.len() - 1].split(':');
            (
                parts
                    .next()
                    .ok_or_else(|| SVDError::InvalidBitRange(tree.id(), InvalidBitRange::Syntax))?
                    .parse::<u32>()
                    .with_context(|| {
                        SVDError::InvalidBitRange(tree.id(), InvalidBitRange::ParseError)
                    })?,
                parts
                    .next()
                    .ok_or_else(|| SVDError::InvalidBitRange(tree.id(), InvalidBitRange::Syntax))?
                    .parse::<u32>()
                    .with_context(|| {
                        SVDError::InvalidBitRange(tree.id(), InvalidBitRange::ParseError)
                    })?,
                BitRangeType::BitRange,
            )
        // TODO: Consider matching instead so we can say which of these tags are missing
        } else if let (Some(lsb), Some(msb)) = (tree.get_child("lsb"), tree.get_child("msb")) {
            (
                // TODO: `u32::parse` should not hide it's errors
                u32::parse(&msb).with_context(|| {
                    SVDError::InvalidBitRange(tree.id(), InvalidBitRange::MsbLsb)
                })?,
                u32::parse(&lsb).with_context(|| {
                    SVDError::InvalidBitRange(tree.id(), InvalidBitRange::MsbLsb)
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
                offset: u32::parse(&offset).with_context(|| {
                    SVDError::InvalidBitRange(tree.id(), InvalidBitRange::ParseError)
                })?,
                width: u32::parse(&width).with_context(|| {
                    SVDError::InvalidBitRange(tree.id(), InvalidBitRange::ParseError)
                })?,
                range_type: BitRangeType::OffsetWidth,
            });
        } else {
            return Err(SVDError::InvalidBitRange(tree.id(), InvalidBitRange::Syntax).into());
        };

        Ok(Self {
            offset: start,
            width: end - start + 1,
            range_type,
        })
    }
}
