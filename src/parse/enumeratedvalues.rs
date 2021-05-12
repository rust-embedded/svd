use super::{elementext::ElementExt, optional, Element, Parse};

use crate::error::*;
use crate::svd::{EnumeratedValue, EnumeratedValues, Usage};

impl Parse for EnumeratedValues {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        assert_eq!(tree.name, "enumeratedValues");
        Ok(EnumeratedValues::builder()
            .name(tree.get_child_text_opt("name")?)
            .usage(optional::<Usage>("usage", tree)?)
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .values({
                let values: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| {
                        ["name", "headerEnumName", "usage"]
                            .iter()
                            .all(|s| &t.name != s)
                    })
                    .enumerate()
                    .map(|(e, t)| {
                        if t.name == "enumeratedValue" {
                            EnumeratedValue::parse(t)
                                .with_context(|| format!("Parsing enumerated value #{}", e))
                        } else {
                            Err(
                                SVDError::NotExpectedTag(t.clone(), "enumeratedValue".to_string())
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
