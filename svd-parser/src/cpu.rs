use super::*;
use crate::svd::{Cpu, Endian};

impl Parse for Cpu {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("cpu") {
            return Err(SVDError::NotExpectedTag("cpu".to_string())
                .at(tree.id())
                .into());
        }

        Cpu::builder()
            .name(tree.get_child_text("name")?)
            .revision(tree.get_child_text("revision")?)
            .endian(Endian::parse(&tree.get_child_elem("endian")?, config)?)
            .mpu_present(tree.get_child_bool("mpuPresent")?)
            .fpu_present(tree.get_child_bool("fpuPresent")?)
            .nvic_priority_bits(tree.get_child_u32("nvicPrioBits")?)
            .has_vendor_systick(tree.get_child_bool("vendorSystickConfig")?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()).into())
    }
}
