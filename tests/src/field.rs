use super::run_test;
use crate::svd::{
    Access, BitRange, BitRangeType, DimElement, EnumeratedValue, EnumeratedValues, Field,
    FieldInfo, ValidateLevel,
};

#[test]
fn decode_encode() {
    let tests = vec![(
        Field::Array(
            FieldInfo::builder()
                .name("MODE%s".to_string())
                .derived_from(Some("other_field".to_string()))
                .bit_range(BitRange::from_offset_width(24, 2))
                .build(ValidateLevel::Strict)
                .unwrap(),
            DimElement::builder()
                .dim(2)
                .dim_increment(4)
                .dim_index(Some(vec!["10".to_string(), "20".to_string()]))
                .build(ValidateLevel::Strict)
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
    run_test::<Field>(&tests[..], None, None);

    let parse_config = svd_parser::Config::default();
    let mut encode_config = svd_encoder::Config::default();
    encode_config.update("field_name", "Snake");
    encode_config.update("field_bit_range", "MsbLsb");

    let tests = vec![(
        FieldInfo::builder()
            .name("MODE".to_string())
            .derived_from(Some("other_field".to_string()))
            .bit_range(BitRange {
                offset: 24,
                width: 2,
                range_type: BitRangeType::OffsetWidth,
            })
            .build(ValidateLevel::Strict)
            .unwrap(),
        "
        <field derivedFrom=\"other_field\">
          <name>MODE</name>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        ",
        "
        <field derivedFrom=\"other_field\">
          <name>mode</name>
          <lsb>24</lsb>
          <msb>25</msb>
        </field>
        ",
    )];

    run_test::<FieldInfo>(&tests[..], Some(parse_config), Some(encode_config));
}

#[test]
fn decode_encode_info() {
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
                        .build(ValidateLevel::Strict)
                        .unwrap()])
                    .build(ValidateLevel::Strict)
                    .unwrap()])
                .build(ValidateLevel::Strict)
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
                .build(ValidateLevel::Strict)
                .unwrap(),
            "
        <field derivedFrom=\"other_field\">
          <name>MODE</name>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        ",
            "
        <field derivedFrom=\"other_field\">
          <name>MODE</name>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        ",
        ),
    ];

    run_test::<FieldInfo>(&tests[..], None, None);
}
