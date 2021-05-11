use super::{elementext::ElementExt, optional, Element, Parse};

use crate::error::*;
use crate::svd::{AddressBlock, Interrupt, Peripheral, RegisterCluster, RegisterProperties};

impl Parse for Peripheral {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "peripheral" {
            return Err(SVDError::NotExpectedTag(tree.clone(), "peripheral".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In peripheral `{}`", name))
    }
}

impl Peripheral {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        Peripheral::builder()
            .name(name)
            .display_name(tree.get_child_text_opt("displayName")?)
            .version(tree.get_child_text_opt("version")?)
            .description(tree.get_child_text_opt("description")?)
            .group_name(tree.get_child_text_opt("groupName")?)
            .base_address(tree.get_child_u64("baseAddress")?)
            .default_register_properties(RegisterProperties::parse(tree)?)
            .address_block(optional::<AddressBlock>("addressBlock", tree)?)
            .interrupt({
                let interrupt: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "interrupt")
                    .enumerate()
                    .map(|(e, i)| {
                        Interrupt::parse(i).with_context(|| format!("Parsing interrupt #{}", e))
                    })
                    .collect();
                interrupt?
            })
            .registers(if let Some(registers) = tree.get_child("registers") {
                let rs: Result<Vec<_>, _> = registers
                    .children
                    .iter()
                    .map(RegisterCluster::parse)
                    .collect();
                Some(rs?)
            } else {
                None
            })
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .build()
    }
}
