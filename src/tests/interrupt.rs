use super::run_test;
use crate::svd::Interrupt;

#[test]
fn decode_encode() {
    let tests = vec![(
        Interrupt {
            name: String::from("test"),
            description: Some(String::from("description")),
            value: 14,
        },
        "
            <interrupt>
                <name>test</name>
                <description>description</description>
                <value>14</value>
            </interrupt>",
    )];

    run_test::<Interrupt>(&tests[..]);
}
