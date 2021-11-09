use super::*;
use crate::svd::{Cpu, Endian};
use crate::types::BoolParse;

impl Parse for Cpu {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("cpu") {
            return Err(SVDError::NotExpectedTag("cpu".to_string()).at(tree.id()));
        }

        Cpu::builder()
            .name(tree.get_child_text("name")?)
            .revision(tree.get_child_text("revision")?)
            .endian(Endian::parse(&tree.get_child_elem("endian")?, config)?)
            .mpu_present(tree.get_child_bool("mpuPresent")?)
            .fpu_present(tree.get_child_bool("fpuPresent")?)
            .fpu_double_precision(optional::<BoolParse>("fpuDP", tree, &())?)
            .dsp_present(optional::<BoolParse>("dspPresent", tree, &())?)
            .icache_present(optional::<BoolParse>("icachePresent", tree, &())?)
            .dcache_present(optional::<BoolParse>("dcachePresent", tree, &())?)
            .itcm_present(optional::<BoolParse>("itcmPresent", tree, &())?)
            .dtcm_present(optional::<BoolParse>("dtcmPresent", tree, &())?)
            .vtor_present(optional::<BoolParse>("vtorPresent", tree, &())?)
            .nvic_priority_bits(tree.get_child_u32("nvicPrioBits")?)
            .has_vendor_systick(tree.get_child_bool("vendorSystickConfig")?)
            .device_num_interrupts(optional::<u32>("deviceNumInterrupts", tree, &())?)
            .sau_num_regions(optional::<u32>("sauNumRegions", tree, &())?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
