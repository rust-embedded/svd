
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

    #[test]
    fn decode_encode() {
        let types = vec![
            (Endian::Little, String::from("<endian>little</endian>")),
            (Endian::Big, String::from("<endian>big</endian>")),
            (Endian::Selectable,String::from("<endian>selectable</endian>")),
            (Endian::Other, String::from("<endian>other</endian>")),
        ];

        for (e, s) in types {
            let tree1 = Element::parse(s.as_bytes()).unwrap();
            let endian = Endian::parse(&tree1).unwrap();
            assert_eq!(endian, e, "Parsing `{}` expected `{:?}`", s, e);
            let tree2 = endian.encode().unwrap();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", e, s);
        }
    }
}
