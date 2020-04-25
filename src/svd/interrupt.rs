#[cfg(feature = "unproven")]
use std::collections::HashMap;

use xmltree::Element;

use crate::elementext::ElementExt;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Interrupt {
    /// The string represents the interrupt name
    pub name: String,

    /// The string describes the interrupt
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Represents the enumeration index value associated to the interrupt
    pub value: u32,
}

impl Interrupt {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        Ok(Self {
            name,
            description: tree.get_child_text_opt("description")?,
            value: tree.get_child_u32("value")?,
        })
    }
}

impl Parse for Interrupt {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "interrupt" {
            return Err(ParseError::NotExpectedTag(tree.clone(), "interrupt".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In interrupt `{}`", name))
    }
}

#[cfg(feature = "unproven")]
impl Encode for Interrupt {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("interrupt"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("description", self.description.clone()),
                new_element("value", Some(format!("{}", self.value))),
            ],
            text: None,
        })
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
}
