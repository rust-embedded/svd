use super::run_test;
use crate::svd::{Interrupt, ValidateLevel};

#[test]
fn decode_encode() {
    let tests = vec![(
        Interrupt::builder()
            .name("test".to_string())
            .description(Some("description".to_string()))
            .value(14)
            .build(ValidateLevel::Strict)
            .unwrap(),
        "
            <interrupt>
                <name>test</name>
                <description>description</description>
                <value>14</value>
            </interrupt>",
        "
            <interrupt>
                <name>test</name>
                <description>description</description>
                <value>14</value>
            </interrupt>",
    )];

    run_test::<Interrupt>(&tests[..], None, None);

    let parse_config = svd_parser::Config::default();
    let mut encode_config = svd_encoder::Config::default();
    encode_config.update("interrupt_name", "Constant");

    let tests = vec![(
        Interrupt::builder()
            .name("test".to_string())
            .description(Some("description".to_string()))
            .value(14)
            .build(ValidateLevel::Strict)
            .unwrap(),
        "
            <interrupt>
                <name>test</name>
                <description>description</description>
                <value>14</value>
            </interrupt>",
        "
            <interrupt>
                <name>TEST</name>
                <description>description</description>
                <value>14</value>
            </interrupt>",
    )];

    run_test::<Interrupt>(&tests[..], Some(parse_config), Some(encode_config));
}
