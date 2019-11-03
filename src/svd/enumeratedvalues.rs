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
#[derive(Clone, Debug, PartialEq, derive_builder::Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct EnumeratedValues {
    /// Identifier for the whole enumeration section
    #[builder(default)]
    pub name: Option<String>,

    #[builder(default)]
    pub usage: Option<Usage>,

    /// Makes a copy from a previously defined enumeratedValues section.
    /// No modifications are allowed
    #[builder(default)]
    pub derived_from: Option<String>,

    pub values: Vec<EnumeratedValue>,

    // Reserve the right to add more fields to this struct
    #[builder(default)]
    _extensible: (),
}

impl EnumeratedValuesBuilder {
    fn validate(&self) -> Result<(), String> {
        match &self.derived_from {
            Some(Some(dname)) if crate::is_valid_name(dname) => Ok(()),
            Some(Some(dname)) => Err(format!("Derive name `{}` is invalid", dname)),
            Some(None) | None => {
                if match &self.values {
                    Some(values) if values.is_empty() => true,
                    None => true,
                    _ => false,
                } {
                    Err("Empty enumerated values".to_string())
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl Parse for EnumeratedValues {
    type Object = EnumeratedValues;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<EnumeratedValues> {
        assert_eq!(tree.name, "enumeratedValues");
        EnumeratedValuesBuilder::default()
            .name(tree.get_child_text_opt("name")?)
            .usage(parse::optional::<Usage>("usage", tree)?)
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .values({
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
                                .context(format!("Parsing enumerated value #{}", e))
                        } else {
                            Err(ParseError::NotExpectedTag(
                                t.clone(),
                                "enumeratedValue".to_string(),
                            )
                            .into())
                        }
                    })
                    .collect();
                values?
            })
            .build()
            .map_err(|e| anyhow::anyhow!(e))
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
    use crate::svd::enumeratedvalue::EnumeratedValueBuilder;

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
                    <value>0x00000001</value>
                </enumeratedValue>
            </enumeratedValues>
        ",
        );

        let expected = EnumeratedValuesBuilder::default()
            .derived_from(Some("fake_derivation".to_string()))
            .values(vec![
                EnumeratedValueBuilder::default()
                    .name("WS0".to_string())
                    .description(Some(
                        "Zero wait-states inserted in fetch or read transfers".to_string()
                    ))
                    .is_default(Some(true))
                    .build()
                    .unwrap(),
                EnumeratedValueBuilder::default()
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
                <value>0x00000000</value>
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
