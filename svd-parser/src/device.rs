use super::*;
use crate::svd::{cpu::Cpu, peripheral::Peripheral, registerproperties::RegisterProperties};

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
            .license_text(tree.get_child_text_opt("licenseText")?)
            .cpu(optional::<Cpu>("cpu", tree, config)?)
            .header_system_filename(tree.get_child_text_opt("headerSystemFilename")?)
            .header_definitions_prefix(tree.get_child_text_opt("headerDefinitionsPrefix")?)
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
        if let Some(version) = tree.get_child_text_opt("version")? {
            device = device.version(version)
        }
        if let Some(description) = tree.get_child_text_opt("description")? {
            device = device.description(description)
        }
        if let Some(bits) = optional::<u32>("addressUnitBits", tree, &())? {
            device = device.address_unit_bits(bits)
        }
        if let Some(width) = optional::<u32>("width", tree, &())? {
            device = device.width(width)
        }
        // TODO: accept namespace other than `xs`
        // Now assert `xs` exists and `noNamespaceSchemaLocation` is under `xs`
        if let Some(xmlns_xs) = tree.lookup_namespace_uri(Some("xs")) {
            device = device.xmlns_xs(xmlns_xs.to_string());
            if let Some(location) = tree.attribute((xmlns_xs, "noNamespaceSchemaLocation")) {
                device = device.no_namespace_schema_location(location.to_string());
            }
        }
        if let Some(schema_version) = tree.attribute("schemaVersion") {
            device = device.schema_version(schema_version.to_string());
        }
        device
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
