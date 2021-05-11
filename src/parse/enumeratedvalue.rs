use super::Element;

use crate::elementext::ElementExt;
use crate::error::*;
use crate::parse;
use crate::svd::EnumeratedValue;
use crate::types::Parse;

impl EnumeratedValue {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        EnumeratedValue::builder()
            .name(name)
            .description(tree.get_child_text_opt("description")?)
            // TODO: this .ok() approach is simple, but does not expose errors parsing child objects.
            // Suggest refactoring all parse::type methods to return result so parse::optional works.
            .value(parse::optional::<u64>("value", tree)?)
            .is_default(tree.get_child_bool("isDefault").ok())
            .build()
    }
}
impl Parse for EnumeratedValue {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "enumeratedValue" {
            return Err(
                SVDError::NotExpectedTag(tree.clone(), "enumeratedValue".to_string()).into(),
            );
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In enumerated value `{}`", name))
    }
}
