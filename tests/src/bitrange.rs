use crate::new_element;
use crate::parse::Parse;
use crate::svd::{BitRange, BitRangeType};
use xmltree::Element;

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
