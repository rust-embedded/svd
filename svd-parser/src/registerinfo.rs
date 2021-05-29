use super::{elementext::ElementExt, optional, Config, Node, Parse, Result, SVDError};
use crate::svd::{Field, ModifiedWriteValues, RegisterInfo, RegisterProperties, WriteConstraint};

impl Parse for RegisterInfo {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self> {
        let name = tree.get_child_text("name")?;
        parse_register(tree, name.clone(), config)
    }
}

fn parse_register(tree: &Node, name: String, config: &Config) -> Result<RegisterInfo> {
    RegisterInfo::builder()
        .name(name)
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
        .map_err(|e| SVDError::from(e).at(tree.id()).into())
}
