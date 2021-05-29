use super::{elementext::ElementExt, optional, Config, Context, Node, Parse, Result, SVDError};
use crate::svd::{EnumeratedValue, EnumeratedValues, Usage};

impl Parse for EnumeratedValues {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self> {
        if !tree.has_tag_name("enumeratedValues") {
            return Err(SVDError::NotExpectedTag("enumeratedValues".to_string())
                .at(tree.id())
                .into());
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
                    .enumerate()
                    .map(|(e, t)| {
                        if t.has_tag_name("enumeratedValue") {
                            EnumeratedValue::parse(&t, config)
                                .with_context(|| format!("Parsing enumerated value #{}", e))
                        } else {
                            Err(SVDError::NotExpectedTag("enumeratedValue".to_string())
                                .at(t.id())
                                .into())
                        }
                    })
                    .collect();
                values?
            })
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()).into())
    }
}
