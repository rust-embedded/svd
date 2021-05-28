use super::{elementext::ElementExt, Node, Parse, Result, SVDError};
use crate::svd::Endian;

impl Parse for Endian {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        let text = tree.get_text()?;

        match &text[..] {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            "selectable" => Ok(Endian::Selectable),
            "other" => Ok(Endian::Other),
            s => Err(SVDError::UnknownEndian(tree.id(), s.into()).into()),
        }
    }
}
