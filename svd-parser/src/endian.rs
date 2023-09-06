use super::*;
use crate::svd::Endian;

impl Parse for Endian {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        let text = trim_spaces(tree.get_text()?, config).map_err(|e| e.at(tree.id()))?;

        Self::parse_str(text).ok_or_else(|| SVDError::UnknownEndian(text.into()).at(tree.id()))
    }
}
