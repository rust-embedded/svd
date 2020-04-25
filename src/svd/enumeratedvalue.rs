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

use crate::Build;

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
    _extensible: (),
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EnumeratedValueError {
    #[error("EnumeratedValue must contain one of `value` (passed {0:?}) or `is_default` (passed {1:?}) tags")]
    AbsentValue(Option<u32>, Option<bool>),
    #[error("Value {0} out of range {1:?}")]
    OutOfRange(u32, core::ops::Range<u32>),
}

impl Build for EnumeratedValue {
    type Builder = EnumeratedValueBuilder;
}

#[derive(Default)]
pub struct EnumeratedValueBuilder {
    name: Option<String>,
    description: Option<String>,
    value: Option<u32>,
    is_default: Option<bool>,
}

impl EnumeratedValueBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn value(mut self, value: Option<u32>) -> Self {
        self.value = value;
        self
    }
    pub fn is_default(mut self, value: Option<bool>) -> Self {
        self.is_default = value;
        self
    }
    pub fn build(self) -> Result<EnumeratedValue> {
        (EnumeratedValue {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description,
            value: self.value,
            is_default: self.is_default,
            _extensible: (),
        })
        .validate()
    }
}

impl EnumeratedValue {
    fn validate(self) -> Result<Self> {
        check_name(&self.name, "name")?;
        match (&self.value, &self.is_default) {
            (Some(_), None) | (None, Some(_)) => Ok(self),
            _ => Err(EnumeratedValueError::AbsentValue(self.value, self.is_default).into()),
        }
    }
    pub(crate) fn check_range(&self, range: &core::ops::Range<u32>) -> Result<()> {
        match &self.value {
            Some(x) if !range.contains(x) => {
                Err(EnumeratedValueError::OutOfRange(*x, range.clone()).into())
            }
            _ => Ok(()),
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
    }
}
impl Parse for EnumeratedValue {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "enumeratedValue" {
            return Err(
                ParseError::NotExpectedTag(tree.clone(), "enumeratedValue".to_string()).into(),
            );
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In enumerated value `{}`", name))
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
