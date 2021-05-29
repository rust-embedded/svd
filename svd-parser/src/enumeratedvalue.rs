use super::*;
use crate::svd::EnumeratedValue;

impl Parse for EnumeratedValue {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("enumeratedValue") {
            return Err(SVDError::NotExpectedTag("enumeratedValue".to_string()).at(tree.id()));
        }

        EnumeratedValue::builder()
            .name(tree.get_child_text("name")?)
            .description(tree.get_child_text_opt("description")?)
            .value(optional::<u64>("value", tree, &())?)
            .is_default(tree.get_child_bool("isDefault").ok())
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
