#[cfg(feature = "unproven")]
use std::collections::HashMap;

use xmltree::Element;

use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::types::Parse;

use crate::error::*;

#[cfg(feature = "serde_svd")]
use super::serde::{ Deserialize, Serialize };

#[cfg_attr(feature = "serde_svd", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
    Selectable,
    Other,
}

impl Parse for Endian {
    type Object = Endian;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Endian, SVDError> {
        let text = tree.get_text()?;

        match &text[..] {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            "selectable" => Ok(Endian::Selectable),
            "other" => Ok(Endian::Other),
            s => Err(SVDErrorKind::UnknownEndian(s.into()).into()),
        }
    }
}

#[cfg(feature = "unproven")]
impl Encode for Endian {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let text = match *self {
            Endian::Little => String::from("little"),
            Endian::Big => String::from("big"),
            Endian::Selectable => String::from("selectable"),
            Endian::Other => String::from("other"),
        };

        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("endian"),
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
            (Endian::Little, "<endian>little</endian>"),
            (Endian::Big, "<endian>big</endian>"),
            (
                Endian::Selectable,
                "<endian>selectable</endian>",
            ),
            (Endian::Other, "<endian>other</endian>"),
        ];

        run_test::<Endian>(&tests[..]);
    }
}
