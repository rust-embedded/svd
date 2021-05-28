use super::run_test;
use crate::svd::EnumeratedValue;

#[test]
fn decode_encode() {
    let tests = vec![(
        EnumeratedValue::builder()
            .name("WS0".to_string())
            .description(Some(
                "Zero wait-states inserted in fetch or read transfers".to_string(),
            ))
            .value(Some(0))
            .build()
            .unwrap(),
        "
            <enumeratedValue>
                <name>WS0</name>
                <description>Zero wait-states inserted in fetch or read transfers</description>
                <value>0</value>
            </enumeratedValue>
        ",
    )];

    run_test::<EnumeratedValue>(&tests[..]);
}
