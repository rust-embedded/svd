use super::{elementext::ElementExt, Config, Node, Parse, Result, SVDError};
use crate::svd::Endian;

impl Parse for Endian {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self> {
        let text = tree.get_text()?;

        match text {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            "selectable" => Ok(Endian::Selectable),
            "other" => Ok(Endian::Other),
            s => Err(SVDError::UnknownEndian(s.into()).at(tree.id()).into()),
        }
    }
}
