use std::collections::HashMap;

use xmltree::Element;
use ElementExt;

use types::{Parse, Encode};
use error::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    Read,
    Write,
    ReadWrite,
}

impl Parse for Usage {
    type Object = Usage;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Usage, SVDError> {
        let text = tree.get_text()?;

        match &text[..] {
            "read" => Ok(Usage::Read),
            "write" => Ok(Usage::Write),
            "read-write" => Ok(Usage::ReadWrite),
            _ => Err(SVDErrorKind::UnknownUsageVariant(tree.clone()).into()),
        }
    }
}

impl Encode for Usage {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let text = match *self {
            Usage::Read => String::from("read"),
            Usage::Write => String::from("write"),
            Usage::ReadWrite => String::from("read-write"),
        };

        Ok(Element {
            name: String::from("usage"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(text),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::test;

    #[test]
    fn decode_encode() {
        let tests = vec![
            (Usage::Read, "<usage>read</usage>"),
            (Usage::Write, "<usage>write</usage>"),
            (Usage::ReadWrite, "<usage>read-write</usage>"),
        ];

        test::<Usage>(&tests[..]);
    }
}
