use super::{elementext::ElementExt, optional, Context, Element, Parse, Result, SVDError};
use crate::svd::EnumeratedValue;

fn parse_ev(tree: &Element, name: String) -> Result<EnumeratedValue> {
    Ok(EnumeratedValue::builder()
        .name(name)
        .description(tree.get_child_text_opt("description")?)
        // TODO: this .ok() approach is simple, but does not expose errors parsing child objects.
        // Suggest refactoring all parse::type methods to return result so parse::optional works.
        .value(optional::<u64>("value", tree)?)
        .is_default(tree.get_child_bool("isDefault").ok())
        .build()?)
}

impl Parse for EnumeratedValue {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if !tree.has_tag_name("enumeratedValue") {
            return Err(SVDError::NotExpectedTag(tree.id(), "enumeratedValue".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        parse_ev(tree, name.clone()).with_context(|| format!("In enumerated value `{}`", name))
    }
}
