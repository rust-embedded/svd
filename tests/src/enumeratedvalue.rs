use super::run_test;
use crate::svd::{EnumeratedValue, ValidateLevel};

#[test]
fn decode_encode() {
    let tests = vec![(
        EnumeratedValue::builder()
            .name("WS0".to_string())
            .description(Some(
                "Zero wait-states inserted in fetch or read transfers".to_string(),
            ))
            .value(Some(0))
            .build(ValidateLevel::Strict)
            .unwrap(),
        "
            <enumeratedValue>
                <name>WS0</name>
                <description>Zero wait-states inserted in fetch or read transfers</description>
                <value>0</value>
            </enumeratedValue>
        ",
        "
            <enumeratedValue>
                <name>WS0</name>
                <description>Zero wait-states inserted in fetch or read transfers</description>
                <value>0</value>
            </enumeratedValue>
        ",
    )];

    run_test::<EnumeratedValue>(&tests[..], None, None);

    let parse_config = svd_parser::Config::default();
    let mut encode_config = svd_encoder::Config::default();
    encode_config.update("enumerated_value_name", "Pascal");
    encode_config.update("enumerated_value_value", "Bin");

    let tests = vec![(
        EnumeratedValue::builder()
            .name("WS0".to_string())
            .description(Some(
                "Zero wait-states inserted in fetch or read transfers".to_string(),
            ))
            .value(Some(0))
            .build(ValidateLevel::Strict)
            .unwrap(),
        "
            <enumeratedValue>
                <name>WS0</name>
                <description>Zero wait-states inserted in fetch or read transfers</description>
                <value>0</value>
            </enumeratedValue>
        ",
        "
            <enumeratedValue>
                <name>Ws0</name>
                <description>Zero wait-states inserted in fetch or read transfers</description>
                <value>0b0</value>
            </enumeratedValue>
        ",
    )];

    run_test::<EnumeratedValue>(&tests[..], Some(parse_config), Some(encode_config));
}
