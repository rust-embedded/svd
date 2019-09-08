#[cfg(feature = "unproven")]
use std::collections::HashMap;

use crate::elementext::ElementExt;
use failure::ResultExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::parse;
use crate::svd::{enumeratedvalue::EnumeratedValue, usage::Usage};
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumeratedValues {
    pub name: Option<String>,
    pub usage: Option<Usage>,
    pub derived_from: Option<String>,
    pub values: Vec<EnumeratedValue>,
}

impl Parse for EnumeratedValues {
    type Object = EnumeratedValues;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<EnumeratedValues, SVDError> {
        assert_eq!(tree.name, "enumeratedValues");

        Ok(EnumeratedValues {
            name: tree.get_child_text_opt("name")?,
            usage: parse::optional::<Usage>("usage", tree)?,
            derived_from: tree.attributes.get("derivedFrom").map(|s| s.to_owned()),
            values: {
                let values: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| {
                        ["name", "headerEnumName", "usage"]
                            .iter()
                            .all(|s| &t.name != s)
                    })
                    .enumerate()
                    .map(|(e, t)| {
                        if t.name == "enumeratedValue" {
                            EnumeratedValue::parse(t).context(SVDErrorKind::Other(format!(
                                "Parsing enumerated value #{}",
                                e
                            )))
                        } else {
                            Err(SVDErrorKind::NotExpectedTag(
                                t.clone(),
                                "enumeratedValue".to_string(),
                            )
                            .into())
                        }
                    })
                    .collect();
                let values = values?;
                if values.is_empty() {
                    return Err(SVDErrorKind::EmptyTag(tree.clone(), tree.name.clone()).into());
                }
                values
            },
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for EnumeratedValues {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let mut base = Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("enumeratedValues"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: None,
        };

        if let Some(d) = &self.name {
            base.children.push(new_element("name", Some((*d).clone())));
        };

        if let Some(v) = &self.usage {
            base.children.push(v.encode()?);
        };

        if let Some(v) = &self.derived_from {
            base.attributes
                .insert(String::from("derivedFrom"), (*v).clone());
        }

        for v in &self.values {
            base.children.push(v.encode()?);
        }

        Ok(base)
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let example = String::from(
            "
            <enumeratedValues derivedFrom=\"fake-derivation.png\">
                <enumeratedValue>
                    <name>WS0</name>
                    <description>Zero wait-states inserted in fetch or read transfers</description>
                    <value>0x00000000</value>
                    <isDefault>true</isDefault>
                </enumeratedValue>
                <enumeratedValue>
                    <name>WS1</name>
                    <description>One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details</description>
                    <value>0x00000001</value>
                </enumeratedValue>
            </enumeratedValues>
        ",
        );

        let expected = EnumeratedValues {
            name: None,
            usage: None,
            derived_from: Some(String::from("fake-derivation.png")),
            values: vec![
                EnumeratedValue {
                    name: String::from("WS0"),
                    description: Some(String::from(
                        "Zero wait-states inserted in fetch or read transfers",
                    )),
                    value: Some(0),
                    is_default: Some(true),
                },
                EnumeratedValue {
                    name: String::from("WS1"),
                    description: Some(String::from(
                        "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details",
                    )),
                    value: Some(1),
                    is_default: None,
                },
            ],
        };

        // TODO: move to test! macro
        let tree1 = Element::parse(example.as_bytes()).unwrap();

        let parsed = EnumeratedValues::parse(&tree1).unwrap();
        assert_eq!(parsed, expected, "Parsing tree failed");

        let tree2 = parsed.encode().unwrap();
        assert_eq!(tree1, tree2, "Encoding value failed");
    }

    #[test]
    fn valid_children() {
        fn parse(contents: String) -> Result<EnumeratedValues, SVDError> {
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
                <value>0x00000000</value>
                <isDefault>true</isDefault>
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
}
