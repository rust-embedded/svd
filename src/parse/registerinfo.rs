use super::{elementext::ElementExt, optional, Element, Parse};

use crate::error::*;
use crate::svd::{Field, ModifiedWriteValues, RegisterInfo, RegisterProperties, WriteConstraint};

impl Parse for RegisterInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In register `{}`", name))
    }
}

impl RegisterInfo {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        Ok(RegisterInfo::builder()
            .name(name)
            .display_name(tree.get_child_text_opt("displayName")?)
            .description(tree.get_child_text_opt("description")?)
            .alternate_group(tree.get_child_text_opt("alternateGroup")?)
            .alternate_register(tree.get_child_text_opt("alternateRegister")?)
            .address_offset(tree.get_child_u32("addressOffset")?)
            .properties(RegisterProperties::parse(tree)?)
            .modified_write_values(optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
            )?)
            .write_constraint(optional::<WriteConstraint>("writeConstraint", tree)?)
            .fields({
                if let Some(fields) = tree.get_child("fields") {
                    let fs: Result<Vec<_>, _> = fields
                        .children
                        .iter()
                        .enumerate()
                        .map(|(e, t)| {
                            Field::parse(t).with_context(|| format!("Parsing field #{}", e))
                        })
                        .collect();
                    Some(fs?)
                } else {
                    None
                }
            })
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .build()?)
    }
}
