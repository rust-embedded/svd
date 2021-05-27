use super::run_test;
use crate::svd::{Access, BitRange, BitRangeType, EnumeratedValue, EnumeratedValues, FieldInfo};

#[test]
fn decode_encode() {
    let tests = vec![
        (
            FieldInfo::builder()
                .name("MODE".to_string())
                .description(Some("Read Mode".to_string()))
                .bit_range(BitRange {
                    offset: 24,
                    width: 2,
                    range_type: BitRangeType::OffsetWidth,
                })
                .access(Some(Access::ReadWrite))
                .enumerated_values(vec![EnumeratedValues::builder()
                    .values(vec![EnumeratedValue::builder()
                        .name("WS0".to_string())
                        .description(Some(
                            "Zero wait-states inserted in fetch or read transfers".to_string(),
                        ))
                        .value(Some(0))
                        .is_default(None)
                        .build()
                        .unwrap()])
                    .build()
                    .unwrap()])
                .build()
                .unwrap(),
            "
        <field>
          <name>MODE</name>
          <description>Read Mode</description>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
          <enumeratedValues>
            <enumeratedValue>
              <name>WS0</name>
              <description>Zero wait-states inserted in fetch or read transfers</description>
              <value>0</value>
            </enumeratedValue>
          </enumeratedValues>
        </field>
        ",
        ),
        (
            FieldInfo::builder()
                .name("MODE".to_string())
                .derived_from(Some("other_field".to_string()))
                .bit_range(BitRange {
                    offset: 24,
                    width: 2,
                    range_type: BitRangeType::OffsetWidth,
                })
                .build()
                .unwrap(),
            "
        <field derivedFrom=\"other_field\">
          <name>MODE</name>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        ",
        ),
    ];

    run_test::<FieldInfo>(&tests[..]);
}
