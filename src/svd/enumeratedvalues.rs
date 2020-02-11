#[cfg(feature = "unproven")]
use std::collections::HashMap;

use crate::elementext::ElementExt;
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
    /// Identifier for the whole enumeration section
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub usage: Option<Usage>,

    /// Makes a copy from a previously defined enumeratedValues section.
    /// No modifications are allowed
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,

    pub values: Vec<EnumeratedValue>,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) _extensible: (),
}

impl Parse for EnumeratedValues {
    type Object = EnumeratedValues;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<EnumeratedValues> {
        assert_eq!(tree.name, "enumeratedValues");
        let derived_from = tree.attributes.get("derivedFrom").map(|s| s.to_owned());
        let is_derived = derived_from.is_some();

        Ok(EnumeratedValues {
            name: tree.get_child_text_opt("name")?,
            usage: parse::optional::<Usage>("usage", tree)?,
            derived_from,
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
                            EnumeratedValue::parse(t)
                                .with_context(|| format!("Parsing enumerated value #{}", e))
                        } else {
                            Err(
                                SVDError::NotExpectedTag(t.clone(), "enumeratedValue".to_string())
                                    .into(),
                            )
                        }
                    })
                    .collect();
                let values = values?;
                if values.is_empty() && !is_derived {
                    return Err(SVDError::EmptyTag(tree.clone(), tree.name.clone()).into());
                }
                values
            },
            _extensible: (),
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for EnumeratedValues {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
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
                    _extensible: (),
                },
                EnumeratedValue {
                    name: String::from("WS1"),
                    description: Some(String::from(
                        "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details",
                    )),
                    value: Some(1),
                    is_default: None,
                    _extensible: (),
                },
            ],
            _extensible: (),
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
