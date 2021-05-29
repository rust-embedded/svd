use super::{elementext::ElementExt, optional, Config, Node, Parse, SVDError, SVDErrorAt};
use crate::svd::{AddressBlock, Interrupt, Peripheral, RegisterCluster, RegisterProperties};

impl Parse for Peripheral {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("peripheral") {
            return Err(SVDError::NotExpectedTag("peripheral".to_string())
                .at(tree.id())
                .into());
        }
        let name = tree.get_child_text("name")?;
        parse_peripheral(tree, name.clone(), config)
    }
}

fn parse_peripheral(tree: &Node, name: String, config: &Config) -> Result<Peripheral, SVDErrorAt> {
    Peripheral::builder()
        .name(name)
        .display_name(tree.get_child_text_opt("displayName")?)
        .version(tree.get_child_text_opt("version")?)
        .description(tree.get_child_text_opt("description")?)
        .group_name(tree.get_child_text_opt("groupName")?)
        .base_address(tree.get_child_u64("baseAddress")?)
        .default_register_properties(RegisterProperties::parse(tree, config)?)
        .address_block(optional::<AddressBlock>("addressBlock", tree, config)?)
        .interrupt({
            let interrupt: Result<Vec<_>, _> = tree
                .children()
                .filter(|t| t.is_element() && t.has_tag_name("interrupt"))
                .map(|i| Interrupt::parse(&i, config))
                .collect();
            interrupt?
        })
        .registers(if let Some(registers) = tree.get_child("registers") {
            let rs: Result<Vec<_>, _> = registers
                .children()
                .filter(Node::is_element)
                .map(|t| RegisterCluster::parse(&t, config))
                .collect();
            Some(rs?)
        } else {
            None
        })
        .derived_from(tree.attribute("derivedFrom").map(|s| s.to_owned()))
        .build(config.validate_level)
        .map_err(|e| SVDError::from(e).at(tree.id()).into())
}
