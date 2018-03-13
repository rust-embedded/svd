
use xmltree::Element;

use parse;
use types::Parse;
use error::SVDError;

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

    fn parse2(tree: &Element) -> Result<Endian, SVDError> {
        let text = parse::get_text(tree)?;

        match &text[..] {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            "selectable" => Ok(Endian::Selectable),
            "other" => Ok(Endian::Other),
            _ => Err(SVDError::UnknownEndian),
        }
    }
}