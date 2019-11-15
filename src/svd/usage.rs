#[cfg(feature = "unproven")]
use std::collections::HashMap;

use crate::elementext::ElementExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    Read,
    Write,
    ReadWrite,
}

impl Parse for Usage {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let text = tree.get_text()?;

        match &text[..] {
            "read" => Ok(Usage::Read),
            "write" => Ok(Usage::Write),
            "read-write" => Ok(Usage::ReadWrite),
            _ => Err(UsageVariantError::Unknown(tree.clone()).into()),
        }
    }
}

#[cfg(feature = "unproven")]
impl Encode for Usage {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let text = match *self {
            Usage::Read => String::from("read"),
            Usage::Write => String::from("write"),
            Usage::ReadWrite => String::from("read-write"),
        };

        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("usage"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(text),
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
        let tests = vec![
            (Usage::Read, "<usage>read</usage>"),
            (Usage::Write, "<usage>write</usage>"),
            (Usage::ReadWrite, "<usage>read-write</usage>"),
        ];

        run_test::<Usage>(&tests[..]);
    }
}
