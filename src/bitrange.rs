extern crate xmltree;

use xmltree::Element;

use helpers::*;
use parse;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BitRangeType {
    BitRange,
    OffsetWidth,
    MsbLsb,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitRange {
    pub offset: u32,
    pub width: u32,
    range_type: BitRangeType
}

impl ParseElem for BitRange {
    fn parse(tree: &Element) -> BitRange {
        let (end, start, range_type): (u32, u32, BitRangeType) = if let Some(range) =
            tree.get_child("bitRange") {
            let text = try!(range.text.as_ref());

            assert!(text.starts_with('['));
            assert!(text.ends_with(']'));

            let mut parts = text[1..text.len() - 1].split(':');

            (try!(try!(parts.next()).parse()), try!(try!(parts.next()).parse()), BitRangeType::BitRange)
        } else if let (Some(lsb), Some(msb)) =
            (tree.get_child("lsb"), tree.get_child("msb")) {
            (try!(parse::u32(msb)), try!(parse::u32(lsb)), BitRangeType::MsbLsb)
        } else {
            return BitRange {
                       offset: try!(parse::u32(try!(tree.get_child("bitOffset")))),
                       width: try!(parse::u32(try!(tree.get_child("bitWidth")))),
                       range_type: BitRangeType::OffsetWidth,
                   };
        };

        BitRange {
            offset: start,
            width: end - start + 1,
            range_type: range_type,
        }
    }
}


impl EncodeChildren for BitRange {
    fn encode_children(&self, elem: &Element) -> Element {
        let mut tree = elem.clone();
        
        let children = match self.range_type {
            BitRangeType::BitRange => vec![
                new_element("bitRange", Some(format!("[{}:{}]", self.offset + self.width - 1, self.offset))),
            ],
            BitRangeType::MsbLsb => vec![
                new_element("lsb", Some(format!("{}", self.offset))),
                new_element("msb", Some(format!("{}", self.offset + self.width - 1)))
            ],
            BitRangeType::OffsetWidth => vec![
                new_element("bitOffset", Some(format!("{}", self.offset))),
                new_element("bitWidth", Some(format!("{}", self.width)))
            ],
        };

        tree.children.append(&mut children.clone());
        tree
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (BitRange{offset: 16, width: 4, range_type: BitRangeType::BitRange}, String::from("
                <fake><bitRange>[19:16]</bitRange></fake>
            ")),
            (BitRange{offset: 16, width: 4, range_type: BitRangeType::OffsetWidth}, String::from("
                <fake><bitOffset>16</bitOffset><bitWidth>4</bitWidth></fake>
            ")),
            (BitRange{offset: 16, width: 4, range_type: BitRangeType::MsbLsb}, String::from("
                <fake><lsb>16</lsb><msb>19</msb></fake>
            ")),
        ];

        for (a, s) in types {
            let tree1 = &try!(Element::parse(s.as_bytes()));
            let value = BitRange::parse(tree1);
            assert_eq!(value, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = new_element("fake", None);
            let tree2 = &value.encode_children(&tree2);
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
