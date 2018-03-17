
use xmltree::Element;
use failure::ResultExt;

use parse;
use types::{Parse, new_element};
use error::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitRange {
    pub offset: u32,
    pub width: u32,
    pub range_type: BitRangeType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BitRangeType {
    BitRange,
    OffsetWidth,
    MsbLsb,
}

impl Parse for BitRange {
    type Object = BitRange;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<BitRange, SVDError> {
        let (end, start, range_type): (u32, u32, BitRangeType) = 
        if let Some(range) = tree.get_child("bitRange") {
            let text = range.text.as_ref().ok_or(SVDErrorKind::Other(format!("text missing")))?; // TODO: Make into a proper error, text empty or something similar
            // TODO: If the `InvalidBitRange` enum was an error we could context into here somehow so that
            // the output would be similar to the parse error
            if !text.starts_with('[') {
                return Err(SVDErrorKind::InvalidBitRange(tree.clone(),InvalidBitRange::Syntax).into()) // TODO: Maybe have a MissingOpen/MissingClosing variant
            }
            if !text.ends_with(']') {
                return Err(SVDErrorKind::InvalidBitRange(tree.clone(),InvalidBitRange::Syntax).into()) // TODO: Maybe have a MissingOpen/MissingClosing variant
            }
 

            let mut parts = text[1..text.len() - 1].split(':');
            (
                parts.next().ok_or(SVDErrorKind::InvalidBitRange(tree.clone(), InvalidBitRange::Syntax))?.parse::<u32>().context(SVDErrorKind::InvalidBitRange(tree.clone(),InvalidBitRange::ParseError))?, 
                parts.next().ok_or(SVDErrorKind::InvalidBitRange(tree.clone(), InvalidBitRange::Syntax))?.parse::<u32>().context(SVDErrorKind::InvalidBitRange(tree.clone(),InvalidBitRange::ParseError))?, 
                BitRangeType::BitRange
            )
        // TODO: Consider matching instead so we can say which of these tags are missing
        } else if let (Some(lsb), Some(msb)) = (tree.get_child("lsb"), tree.get_child("msb")) {
            (
                // TODO: `parse::u32` should not hide it's errors
                parse::u32(msb).ok_or(SVDErrorKind::InvalidBitRange(tree.clone(), InvalidBitRange::MsbLsb))?, 
                parse::u32(lsb).ok_or(SVDErrorKind::InvalidBitRange(tree.clone(), InvalidBitRange::MsbLsb))?, 
                BitRangeType::MsbLsb
            )
        } else if let (Some(offset), Some(width)) = (tree.get_child("bitOffset"), tree.get_child("bitWidth")) {
            (
                // Special case because offset and width are directly provided
                // (ie. do not need to be calculated as in the final step)
                return Ok(BitRange {
                    // TODO: capture that error comes from offset/width tag
                    // TODO: `parse::u32` should not hide it's errors
                    offset: parse::u32(offset).ok_or(SVDErrorKind::InvalidBitRange(tree.clone(), InvalidBitRange::ParseError))?, 
                    width: parse::u32(width).ok_or(SVDErrorKind::InvalidBitRange(tree.clone(), InvalidBitRange::ParseError))?, 
                    range_type: BitRangeType::OffsetWidth
                })
            )
        } else {
            return Err(SVDErrorKind::InvalidBitRange(tree.clone(), InvalidBitRange::Syntax).into())   
        };

        Ok(BitRange {
            offset: start,
            width: end - start + 1,
            range_type: range_type,
        })
    }
}

impl BitRange {
    // Encode method differs as it acts on a set of possible children
    // TODO: create interface or decide how to better do this
    pub fn encode(&self) -> Result<Vec<Element>, SVDError> {
        match self.range_type {
            BitRangeType::BitRange => {
                Ok(vec![
                    new_element(
                        "bitRange",
                        Some(format!(
                            "[{}:{}]",
                            self.offset + self.width - 1,
                            self.offset
                        ))
                    ),
                ])
            }
            BitRangeType::MsbLsb => {
                Ok(vec![
                    new_element("lsb", Some(format!("{}", self.offset))),
                    new_element("msb", Some(format!("{}", self.offset + self.width - 1))),
                ])
            }
            BitRangeType::OffsetWidth => {
                Ok(vec![
                    new_element("bitOffset", Some(format!("{}", self.offset))),
                    new_element("bitWidth", Some(format!("{}", self.width))),
                ])
            }
        }
    }
}

#[cfg(test)]
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
                )
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
                )
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
                )
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
