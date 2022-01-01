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

        let mut device = Device::builder()
            .vendor(tree.get_child_text_opt("vendor")?)
            .vendor_id(tree.get_child_text_opt("vendorID")?)
            .name(tree.get_child_text("name")?)
            .series(tree.get_child_text_opt("series")?)
            .version(tree.get_child_text("version")?)
            .description(tree.get_child_text("description")?)
            .license_text(tree.get_child_text_opt("licenseText")?)
            .cpu(optional::<Cpu>("cpu", tree, config)?)
            .header_system_filename(tree.get_child_text_opt("headerSystemFilename")?)
            .header_definitions_prefix(tree.get_child_text_opt("headerDefinitionsPrefix")?)
            .address_unit_bits(tree.get_child_u32("addressUnitBits")?)
            .width(tree.get_child_u32("width")?)
            .default_register_properties(RegisterProperties::parse(tree, config)?)
            .peripherals({
                let ps: Result<Vec<_>, _> = tree
                    .get_child_elem("peripherals")?
                    .children()
                    .filter(Node::is_element)
                    .map(|t| Peripheral::parse(&t, config))
                    .collect();
                ps?
            });
        if let Some(xmlns_xs) = tree.attribute("xmlns:xs") {
            device = device.xmlns_xs(xmlns_xs.to_string());
        }
        if let Some(location) = tree.attribute("xs:noNamespaceSchemaLocation") {
            device = device.no_namespace_schema_location(location.to_string());
        }
        if let Some(schema_version) = tree.attribute("schemaVersion") {
            device = device.schema_version(schema_version.to_string());
        }
        device
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
