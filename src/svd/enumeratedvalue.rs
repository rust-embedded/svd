#[cfg(feature = "unproven")]
use std::collections::HashMap;

use crate::elementext::ElementExt;
use crate::parse;
use xmltree::Element;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::types::Parse;

use crate::FlatRef;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, derive_builder::Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct EnumeratedValue {
    /// String describing the semantics of the value. Can be displayed instead of the value
    pub name: String,

    /// Extended string describing the value
    #[builder(default)]
    pub description: Option<String>,

    /// Defines the constant for the bit-field as decimal, hexadecimal or binary number
    #[builder(default)]
    pub value: Option<u32>,

    /// Defines the name and description for all other values that are not listed explicitly
    #[builder(default)]
    pub is_default: Option<bool>,

    // Reserve the right to add more fields to this struct
    #[builder(default)]
    _extensible: (),
}

impl EnumeratedValueBuilder {
    fn validate(&self) -> Result<(), String> {
        match &self.name {
            Some(name) if crate::is_valid_name(name) => Ok(()),
            Some(name) => Err(format!("EnumeratedValue name `{}` is invalid", name)),
            None => Err("EnumeratedValue must have name".to_string()),
        }?;
        match (self.value.flatref(), self.is_default.flatref()) {
            (Some(_), None) | (None, Some(_)) => Ok(()),
            _ => Err(format!(
                "EnumeratedValue must contain one of `value` ({:?}) or `is_default` ({:?}) tags",
                self.value, self.is_default
            )),
        }
    }
}

impl EnumeratedValue {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        EnumeratedValueBuilder::default()
            .name(name)
            .description(tree.get_child_text_opt("description")?)
            // TODO: this .ok() approach is simple, but does not expose errors parsing child objects.
            // Suggest refactoring all parse::type methods to return result so parse::optional works.
            .value(parse::optional::<u32>("value", tree)?)
            .is_default(tree.get_child_bool("isDefault").ok())
            .build()
            .map_err(|e| anyhow::anyhow!(e))
    }
}
impl Parse for EnumeratedValue {
    type Object = EnumeratedValue;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "enumeratedValue" {
            return Err(
                ParseError::NotExpectedTag(tree.clone(), "enumeratedValue".to_string()).into(),
            );
        }
        let name = tree.get_child_text("name")?;
        EnumeratedValue::_parse(tree, name.clone())
            .with_context(|| format!("In enumerated value `{}`", name))
    }
}

#[cfg(feature = "unproven")]
impl Encode for EnumeratedValue {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut base = Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("enumeratedValue"),
            attributes: HashMap::new(),
            children: vec![new_element("name", Some(self.name.clone()))],
            text: None,
        };

        if let Some(d) = &self.description {
            let s = (*d).clone();
            base.children.push(new_element("description", Some(s)));
        };

        if let Some(v) = &self.value {
            base.children
                .push(new_element("value", Some(format!("0x{:08.x}", *v))));
        };

        if let Some(v) = &self.is_default {
            base.children
                .push(new_element("isDefault", Some(format!("{}", v))));
        };

        Ok(base)
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            EnumeratedValueBuilder::default()
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
                    <value>0x00000000</value>
                </enumeratedValue>
            ",
        )];

        run_test::<EnumeratedValue>(&tests[..]);
    }
}
