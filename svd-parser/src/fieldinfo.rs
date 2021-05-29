use super::{elementext::ElementExt, optional, Config, Node, Parse, Result, SVDError};
use crate::svd::{
    Access, BitRange, EnumeratedValues, FieldInfo, ModifiedWriteValues, WriteConstraint,
};

impl Parse for FieldInfo {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self> {
        if !tree.has_tag_name("field") {
            return Err(SVDError::NotExpectedTag("field".to_string())
                .at(tree.id())
                .into());
        }
        let name = tree.get_child_text("name")?;
        parse_field(tree, name.clone(), config)
    }
}

fn parse_field(tree: &Node, name: String, config: &Config) -> Result<FieldInfo> {
    let bit_range = BitRange::parse(tree, config)?;
    FieldInfo::builder()
        .name(name)
        .description(tree.get_child_text_opt("description")?)
        .bit_range(bit_range)
        .access(optional::<Access>("access", tree, config)?)
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
        .enumerated_values({
            let values: Result<Vec<_>, _> = tree
                .children()
                .filter(|t| t.is_element() && t.has_tag_name("enumeratedValues"))
                .map(|t| EnumeratedValues::parse(&t, config))
                .collect();
            values?
        })
        .derived_from(tree.attribute("derivedFrom").map(|s| s.to_owned()))
        .build(config.validate_level)
        .map_err(|e| SVDError::from(e).at(tree.id()).into())
}
