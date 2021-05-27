use super::{elementext::ElementExt, optional, Context, Element, Parse, Result, SVDError};
use crate::svd::{EnumeratedValue, EnumeratedValues, Usage};

impl Parse for EnumeratedValues {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        assert!(tree.has_tag_name("enumeratedValues"));
        Ok(EnumeratedValues::builder()
            .name(tree.get_child_text_opt("name")?)
            .usage(optional::<Usage>("usage", tree)?)
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
                            EnumeratedValue::parse(&t)
                                .with_context(|| format!("Parsing enumerated value #{}", e))
                        } else {
                            Err(
                                SVDError::NotExpectedTag(t.id(), "enumeratedValue".to_string())
                                    .into(),
                            )
                        }
                    })
                    .collect();
                values?
            })
            .build()?)
    }
}
