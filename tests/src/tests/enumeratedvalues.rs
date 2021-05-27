use crate::encode::Encode;
use crate::parse::Parse;
use crate::svd::{EnumeratedValue, EnumeratedValues};
use anyhow::Result;
use xmltree::Element;

#[test]
fn decode_encode() {
    let example = String::from(
        "
        <enumeratedValues derivedFrom=\"fake_derivation\">
            <enumeratedValue>
                <name>WS0</name>
                <description>Zero wait-states inserted in fetch or read transfers</description>
                <isDefault>true</isDefault>
            </enumeratedValue>
            <enumeratedValue>
                <name>WS1</name>
                <description>One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details</description>
                <value>1</value>
            </enumeratedValue>
        </enumeratedValues>
    ",
    );

    let expected = EnumeratedValues::builder()
        .derived_from(Some("fake_derivation".to_string()))
        .values(vec![
            EnumeratedValue::builder()
                .name("WS0".to_string())
                .description(Some(
                    "Zero wait-states inserted in fetch or read transfers".to_string()
                ))
                .is_default(Some(true))
                .build()
                .unwrap(),
            EnumeratedValue::builder()
                .name("WS1".to_string())
                .description(Some(
                    "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details".to_string()
                ))
                .value(Some(1))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap();

    // TODO: move to test! macro
    let tree1 = Element::parse(example.as_bytes()).unwrap();

    let parsed = EnumeratedValues::parse(&tree1).unwrap();
    assert_eq!(parsed, expected, "Parsing tree failed");

    let tree2 = parsed.encode().unwrap();
    assert_eq!(tree1, tree2, "Encoding value failed");
}

#[test]
fn valid_children() {
    fn parse(contents: String) -> Result<EnumeratedValues> {
        let example = String::from("<enumeratedValues>") + &contents + "</enumeratedValues>";
        let tree = Element::parse(example.as_bytes()).unwrap();
        EnumeratedValues::parse(&tree)
    }

    // `enumeratedValue` occurrence: 1..*
    parse("".into()).expect_err("must contain at least one <enumeratedValue>");

    let value = String::from(
        "
        <enumeratedValue>
            <name>WS0</name>
            <description>Zero wait-states inserted in fetch or read transfers</description>
            <value>0</value>
        </enumeratedValue>",
    );

    // Valid tags
    parse(value.clone() + "<name>foo</name>").expect("<name> is valid");
    parse(value.clone() + "<headerEnumName>foo</headerEnumName>")
        .expect("<headerEnumName> is valid");
    parse(value.clone() + "<usage>read</usage>").expect("<usage> is valid");

    // Invalid tags
    parse(value.clone() + "<enumerateValue></enumerateValue>")
        .expect_err("<enumerateValue> in invalid here");
    parse(value.clone() + "<enumeratedValues></enumeratedValues>")
        .expect_err("<enumeratedValues> in invalid here");
}
