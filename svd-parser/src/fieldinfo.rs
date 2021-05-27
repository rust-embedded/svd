use super::{elementext::ElementExt, optional, Context, Element, Parse, Result, SVDError};
use crate::svd::{
    Access, BitRange, EnumeratedValues, FieldInfo, ModifiedWriteValues, WriteConstraint,
};

impl Parse for FieldInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "field" {
            return Err(SVDError::NotExpectedTag(tree.clone(), "field".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        parse_field(tree, name.clone()).with_context(|| format!("In field `{}`", name))
    }
}

fn parse_field(tree: &Element, name: String) -> Result<FieldInfo> {
    let bit_range = BitRange::parse(tree)?;
    Ok(FieldInfo::builder()
        .name(name)
        .description(tree.get_child_text_opt("description")?)
        .bit_range(bit_range)
        .access(optional::<Access>("access", tree)?)
        .modified_write_values(optional::<ModifiedWriteValues>(
            "modifiedWriteValues",
            tree,
        )?)
        .write_constraint(optional::<WriteConstraint>("writeConstraint", tree)?)
        .enumerated_values({
            let values: Result<Vec<_>, _> = tree
                .children
                .iter()
                .filter(|t| t.name == "enumeratedValues")
                .map(|t| EnumeratedValues::parse(t))
                .collect();
            values?
        })
        .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
        .build()?)
}
