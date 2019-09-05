#[cfg(feature = "unproven")]
use std::collections::HashMap;

use failure::ResultExt;
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
    pub name: String,
    pub description: Option<String>,
    pub value: u32,
}

impl Interrupt {
    fn _parse(tree: &Element, name: String) -> Result<Interrupt, SVDError> {
        Ok(Interrupt {
            name,
            description: tree.get_child_text_opt("description")?,
            value: tree.get_child_u32("value")?,
        })
    }
}

impl Parse for Interrupt {
    type Object = Interrupt;
    type Error = SVDError;
    fn parse(tree: &Element) -> Result<Interrupt, SVDError> {
        if tree.name != "interrupt" {
            return Err(SVDErrorKind::NotExpectedTag(
                tree.clone(),
                format!("interrupt"),
            ).into());
        }
        let name = tree.get_child_text("name")?;
        Interrupt::_parse(tree, name.clone())
            .context(SVDErrorKind::Other(format!(
                "In interrupt `{}`",
                name
            )))
            .map_err(|e| e.into())
    }
}

#[cfg(feature = "unproven")]
impl Encode for Interrupt {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
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
