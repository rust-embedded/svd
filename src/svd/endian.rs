
use std::collections::HashMap;

use xmltree::Element;

use parse;
use types::{Parse, Encode};
use error::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
    Selectable,
    Other
}

impl Parse for Endian {
    type Object = Endian;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Endian, SVDError> {
        let text = parse::get_text(tree)?;

        match &text[..] {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            "selectable" => Ok(Endian::Selectable),
            "other" => Ok(Endian::Other),
            s => Err(SVDErrorKind::UnknownEndian(s.into()).into()),
        }
    }
}

impl Encode for Endian {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let text = match *self {
            Endian::Little => String::from("little"),
            Endian::Big => String::from("big"),
            Endian::Selectable => String::from("selectable"),
            Endian::Other => String::from("other")
        };

        Ok(Element {
            name: String::from("endian"),
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
            (Endian::Little, "<endian>little</endian>"),
            (Endian::Big, "<endian>big</endian>"),
            (Endian::Selectable,"<endian>selectable</endian>"),
            (Endian::Other, "<endian>other</endian>"),
        ];

        test::<Endian>(&tests[..]);
    }
}
