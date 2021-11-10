use super::*;
use crate::svd::Interrupt;

impl Parse for Interrupt {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("interrupt") {
            return Err(SVDError::NotExpectedTag("interrupt".to_string()).at(tree.id()));
        }
        let name = tree.get_child_text("name")?;

        Interrupt::builder()
            .name(name)
            .description(tree.get_child_text_opt("description")?)
            .value(tree.get_child_u32("value")?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
