use super::*;

use crate::svd::Access;
impl Parse for Access {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self, Self::Error> {
        let text = tree.get_text()?;

        Self::parse_str(text).ok_or_else(|| SVDError::UnknownAccessType(text.into()).at(tree.id()))
    }
}
