use super::*;
use crate::svd::{
    cpu::Cpu, peripheral::Peripheral, registerproperties::RegisterProperties, Device,
};

/// Parses a SVD file
impl Parse for Device {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("device") {
            return Err(SVDError::NotExpectedTag("device".to_string()).at(tree.id()));
        }

        Device::builder()
            .name(tree.get_child_text("name")?)
            .version(tree.get_child_text_opt("version")?)
            .description(tree.get_child_text_opt("description")?)
            .cpu(optional::<Cpu>("cpu", tree, config)?)
            .address_unit_bits(optional::<u32>("addressUnitBits", tree, &())?)
            .width(optional::<u32>("width", tree, &())?)
            .default_register_properties(RegisterProperties::parse(tree, config)?)
            .peripherals({
                let ps: Result<Vec<_>, _> = tree
                    .get_child_elem("peripherals")?
                    .children()
                    .filter(Node::is_element)
                    .map(|t| Peripheral::parse(&t, config))
                    .collect();
                ps?
            })
            .schema_version(tree.attribute("schemaVersion").map(|s| s.to_string()))
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
