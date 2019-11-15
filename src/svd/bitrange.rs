use xmltree::Element;

use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitRange {
    /// Value defining the position of the least significant bit of the field within the register
    pub offset: u32,

    /// Value defining the bit-width of the bitfield within the register
    pub width: u32,

    pub range_type: BitRangeType,
}

impl BitRange {
    pub fn lsb(&self) -> u32 {
        self.offset
    }
    pub fn msb(&self) -> u32 {
        self.offset + self.width - 1
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BitRangeType {
    BitRange,
    OffsetWidth,
    MsbLsb,
}

impl Parse for BitRange {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let (end, start, range_type): (u32, u32, BitRangeType) = if let Some(range) =
            tree.get_child("bitRange")
        {
            let text = range
                .text
                .as_ref()
                .ok_or_else(|| BitRangeError::Invalid(tree.clone(), InvalidBitRange::Empty))?;
            if !text.starts_with('[') {
                return Err(BitRangeError::Invalid(tree.clone(), InvalidBitRange::Syntax).into());
                // TODO: Maybe have a MissingOpen/MissingClosing variant
            }
            if !text.ends_with(']') {
                return Err(BitRangeError::Invalid(tree.clone(), InvalidBitRange::Syntax).into());
                // TODO: Maybe have a MissingOpen/MissingClosing variant
            }

            let mut parts = text[1..text.len() - 1].split(':');
            (
                parts
                    .next()
                    .ok_or_else(|| BitRangeError::Invalid(tree.clone(), InvalidBitRange::Syntax))?
                    .parse::<u32>()
                    .with_context(|| {
                        BitRangeError::Invalid(tree.clone(), InvalidBitRange::ParseError)
                    })?,
                parts
                    .next()
                    .ok_or_else(|| BitRangeError::Invalid(tree.clone(), InvalidBitRange::Syntax))?
                    .parse::<u32>()
                    .with_context(|| {
                        BitRangeError::Invalid(tree.clone(), InvalidBitRange::ParseError)
                    })?,
                BitRangeType::BitRange,
            )
        // TODO: Consider matching instead so we can say which of these tags are missing
        } else if let (Some(lsb), Some(msb)) = (tree.get_child("lsb"), tree.get_child("msb")) {
            (
                // TODO: `u32::parse` should not hide it's errors
                u32::parse(msb).with_context(|| {
                    BitRangeError::Invalid(tree.clone(), InvalidBitRange::MsbLsb)
                })?,
                u32::parse(lsb).with_context(|| {
                    BitRangeError::Invalid(tree.clone(), InvalidBitRange::MsbLsb)
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
                offset: u32::parse(offset).with_context(|| {
                    BitRangeError::Invalid(tree.clone(), InvalidBitRange::ParseError)
                })?,
                width: u32::parse(width).with_context(|| {
                    BitRangeError::Invalid(tree.clone(), InvalidBitRange::ParseError)
                })?,
                range_type: BitRangeType::OffsetWidth,
            });
        } else {
            return Err(BitRangeError::Invalid(tree.clone(), InvalidBitRange::Syntax).into());
        };

        Ok(Self {
            offset: start,
            width: end - start + 1,
            range_type,
        })
    }
}
#[cfg(feature = "unproven")]
impl BitRange {
    // TODO: Encode method differs from Encode trait as it acts on a set of possible children, create an interface or decide how to better do this
    pub fn encode(&self) -> Result<Vec<Element>> {
        match self.range_type {
            BitRangeType::BitRange => Ok(vec![new_element(
                "bitRange",
                Some(format!("[{}:{}]", self.msb(), self.lsb())),
            )]),
            BitRangeType::MsbLsb => Ok(vec![
                new_element("lsb", Some(format!("{}", self.lsb()))),
                new_element("msb", Some(format!("{}", self.msb()))),
            ]),
            BitRangeType::OffsetWidth => Ok(vec![
                new_element("bitOffset", Some(format!("{}", self.offset))),
                new_element("bitWidth", Some(format!("{}", self.width))),
            ]),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (
                BitRange {
                    offset: 16,
                    width: 4,
                    range_type: BitRangeType::BitRange,
                },
                String::from(
                    "
                <fake><bitRange>[19:16]</bitRange></fake>
            ",
                ),
            ),
            (
                BitRange {
                    offset: 16,
                    width: 4,
                    range_type: BitRangeType::OffsetWidth,
                },
                String::from(
                    "
                <fake><bitOffset>16</bitOffset><bitWidth>4</bitWidth></fake>
            ",
                ),
            ),
            (
                BitRange {
                    offset: 16,
                    width: 4,
                    range_type: BitRangeType::MsbLsb,
                },
                String::from(
                    "
                <fake><lsb>16</lsb><msb>19</msb></fake>
            ",
                ),
            ),
        ];

        for (a, s) in types {
            let tree1 = Element::parse(s.as_bytes()).unwrap();
            let value = BitRange::parse(&tree1).unwrap();
            assert_eq!(value, a, "Parsing `{}` expected `{:?}`", s, a);
            let mut tree2 = new_element("fake", None);
            tree2.children = value.encode().unwrap();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
