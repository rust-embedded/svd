use crate::NS;
use minidom::Element;

use crate::elementext::ElementExt;
use crate::encode::Encode;
use crate::types::Parse;

use crate::error::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
    Selectable,
    Other,
}

impl Parse for Endian {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let text = tree.get_text()?;

        match &text[..] {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            "selectable" => Ok(Endian::Selectable),
            "other" => Ok(Endian::Other),
            s => Err(SVDError::UnknownEndian(s.into()).into()),
        }
    }
}

impl Encode for Endian {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let text = match *self {
            Endian::Little => String::from("little"),
            Endian::Big => String::from("big"),
            Endian::Selectable => String::from("selectable"),
            Endian::Other => String::from("other"),
        };

        Ok(Element::builder("endian", NS).append(text).build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![
            (Endian::Little, "<endian>little</endian>"),
            (Endian::Big, "<endian>big</endian>"),
            (Endian::Selectable, "<endian>selectable</endian>"),
            (Endian::Other, "<endian>other</endian>"),
        ];

        run_test::<Endian>(&tests[..]);
    }
}
