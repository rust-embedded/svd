
use xmltree::Element;

use ::parse;
use ::types::{Parse, new_element};
use ::error::SVDError;


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

    // TODO: fix error handling here, haven't got the cycles to work it out right now
    fn parse(tree: &Element) -> Result<BitRange, SVDError> {
        let (end, start, range_type): (u32, u32, BitRangeType) = 
        if let Some(range) = tree.get_child("bitRange") {
            let text = range.text.as_ref().unwrap();

            assert!(text.starts_with('['));
            assert!(text.ends_with(']'));

            let mut parts = text[1..text.len() - 1].split(':');
            (
                (parts.next()).unwrap().parse().unwrap(), 
                (parts.next()).unwrap().parse().unwrap(),
                BitRangeType::BitRange
            )
        } else if let (Some(lsb), Some(msb)) = (tree.get_child("lsb"), tree.get_child("msb")) {
            (
                parse::u32(msb).unwrap(), 
                parse::u32(lsb).unwrap(), 
                BitRangeType::MsbLsb
            )
        } else if let (Some(offset), Some(width)) = (tree.get_child("bitOffset"), tree.get_child("bitWidth")) {
            (
                // Special case because offset and width are directly provided
                // (ie. do not need to be calculated as in the final step)
                return Ok(BitRange {
                    offset: parse::u32(offset).unwrap(), 
                    width: parse::u32(width).unwrap(), 
                    range_type: BitRangeType::OffsetWidth
                })
            )
        } else {
            return Err(SVDError::InvalidBitRange(tree.clone()))   
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
