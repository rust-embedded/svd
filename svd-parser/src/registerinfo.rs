use super::*;
use crate::svd::{
    Field, ModifiedWriteValues, ReadAction, RegisterInfo, RegisterProperties, WriteConstraint,
};

impl Parse for RegisterInfo {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        RegisterInfo::builder()
            .name(tree.get_child_text("name")?)
            .display_name(tree.get_child_text_opt("displayName")?)
            .description(tree.get_child_text_opt("description")?)
            .alternate_group(tree.get_child_text_opt("alternateGroup")?)
            .alternate_register(tree.get_child_text_opt("alternateRegister")?)
            .address_offset(tree.get_child_u32("addressOffset")?)
            .properties(RegisterProperties::parse(tree, config)?)
            .modified_write_values(optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
                config,
            )?)
            .write_constraint(optional::<WriteConstraint>(
                "writeConstraint",
                tree,
                config,
            )?)
            .read_action(optional::<ReadAction>("readAction", tree, config)?)
            .fields({
                if let Some(fields) = tree.get_child("fields") {
                    let fs: Result<Vec<_>, _> = fields
                        .children()
                        .filter(Node::is_element)
                        .map(|t| Field::parse(&t, config))
                        .collect();
                    Some(fs?)
                } else {
                    None
                }
            })
            .derived_from(tree.attribute("derivedFrom").map(|s| s.to_owned()))
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
