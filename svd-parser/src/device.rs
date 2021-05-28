use super::{elementext::ElementExt, optional, Context, Node, Parse, Result, SVDError};
use crate::svd::{
    cpu::Cpu, peripheral::Peripheral, registerproperties::RegisterProperties, Device,
};

/// Parses a SVD file
impl Parse for Device {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        if !tree.has_tag_name("device") {
            return Err(SVDError::NotExpectedTag(tree.id(), "device".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        parse_device(tree, name.clone()).with_context(|| format!("In device `{}`", name))
    }
}

fn parse_device(tree: &Node, name: String) -> Result<Device> {
    Ok(Device::builder()
        .name(name)
        .version(tree.get_child_text_opt("version")?)
        .description(tree.get_child_text_opt("description")?)
        .cpu(optional::<Cpu>("cpu", tree)?)
        .address_unit_bits(optional::<u32>("addressUnitBits", tree)?)
        .width(optional::<u32>("width", tree)?)
        .default_register_properties(RegisterProperties::parse(tree)?)
        .peripherals({
            let ps: Result<Vec<_>, _> = tree
                .get_child_elem("peripherals")?
                .children()
                .filter(Node::is_element)
                .map(|t| Peripheral::parse(&t))
                .collect();
            ps?
        })
        .schema_version(tree.attribute("schemaVersion").map(|s| s.to_string()))
        .build()?)
}
