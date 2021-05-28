use super::run_test;
use crate::svd::{BitRange, BitRangeType, DimElement, Field, FieldInfo};

#[test]
fn decode_encode() {
    let tests = vec![(
        Field::Array(
            FieldInfo::builder()
                .name("MODE%s".to_string())
                .derived_from(Some("other_field".to_string()))
                .bit_range(BitRange {
                    offset: 24,
                    width: 2,
                    range_type: BitRangeType::OffsetWidth,
                })
                .build()
                .unwrap(),
            DimElement::builder()
                .dim(2)
                .dim_increment(4)
                .dim_index(Some(vec!["10".to_string(), "20".to_string()]))
                .build()
                .unwrap(),
        ),
        "
        <field derivedFrom=\"other_field\">
          <dim>2</dim>
          <dimIncrement>0x4</dimIncrement>
          <dimIndex>10,20</dimIndex>
          <name>MODE%s</name>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        ",
    )];
    run_test::<Field>(&tests[..]);
}
