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

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumeratedValue {
    /// String describing the semantics of the value. Can be displayed instead of the value
    pub name: String,

    /// Extended string describing the value
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Defines the constant for the bit-field as decimal, hexadecimal or binary number
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub value: Option<u32>,

    /// Defines the name and description for all other values that are not listed explicitly
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub is_default: Option<bool>,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) _extensible: (),
}
impl EnumeratedValue {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        Ok(Self {
            name,
            description: tree.get_child_text_opt("description")?,
            // TODO: this .ok() approach is simple, but does not expose errors parsing child objects.
            // Suggest refactoring all parse::type methods to return result so parse::optional works.
            value: parse::optional::<u32>("value", tree)?,
            is_default: tree.get_child_bool("isDefault").ok(),
            _extensible: (),
        })
    }
}
impl Parse for EnumeratedValue {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "enumeratedValue" {
            return Err(
                SVDError::NotExpectedTag(tree.clone(), "enumeratedValue".to_string()).into(),
            );
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone())
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
            EnumeratedValue {
                name: String::from("WS0"),
                description: Some(String::from(
                    "Zero wait-states inserted in fetch or read transfers",
                )),
                value: Some(0),
                is_default: Some(true),
                _extensible: (),
            },
            "
                <enumeratedValue>
                    <name>WS0</name>
                    <description>Zero wait-states inserted in fetch or read transfers</description>
                    <value>0x00000000</value>
                    <isDefault>true</isDefault>
                </enumeratedValue>
            ",
        )];

        run_test::<EnumeratedValue>(&tests[..]);
    }
}
