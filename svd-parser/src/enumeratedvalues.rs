use super::*;
use crate::svd::{EnumeratedValue, EnumeratedValues, Usage};

impl Parse for EnumeratedValues {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("enumeratedValues") {
            return Err(SVDError::NotExpectedTag("enumeratedValues".to_string()).at(tree.id()));
        }
        EnumeratedValues::builder()
            .name(tree.get_child_text_opt("name")?)
            .usage(optional::<Usage>("usage", tree, config)?)
            .derived_from(tree.attribute("derivedFrom").map(|s| s.to_owned()))
            .values({
                let values: Result<Vec<_>, _> = tree
                    .children()
                    .filter(|t| {
                        t.is_element()
                            && !matches!(t.tag_name().name(), "name" | "headerEnumName" | "usage")
                    })
                    .map(|t| {
                        if t.has_tag_name("enumeratedValue") {
                            EnumeratedValue::parse(&t, config)
                        } else {
                            Err(SVDError::NotExpectedTag("enumeratedValue".to_string()).at(t.id()))
                        }
                    })
                    .collect();
                values?
            })
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
