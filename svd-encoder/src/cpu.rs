use super::{new_node, Element, Encode, EncodeError};

use crate::svd::Cpu;
impl Encode for Cpu {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut children = vec![
            new_node("name", self.name.clone()),
            new_node("revision", self.revision.clone()),
            self.endian.encode_node()?,
            new_node("mpuPresent", format!("{}", self.mpu_present)),
            new_node("fpuPresent", format!("{}", self.fpu_present)),
        ];
        if let Some(v) = &self.fpu_double_precision {
            children.push(new_node("fpuDP", format!("{}", v)));
        }
        if let Some(v) = &self.dsp_present {
            children.push(new_node("dspPresent", format!("{}", v)));
        }
        if let Some(v) = &self.icache_present {
            children.push(new_node("icachePresent", format!("{}", v)));
        }
        if let Some(v) = &self.dcache_present {
            children.push(new_node("dcachePresent", format!("{}", v)));
        }
        if let Some(v) = &self.itcm_present {
            children.push(new_node("itcmPresent", format!("{}", v)));
        }
        if let Some(v) = &self.dtcm_present {
            children.push(new_node("dtcmPresent", format!("{}", v)));
        }
        if let Some(v) = &self.vtor_present {
            children.push(new_node("vtorPresent", format!("{}", v)));
        }
        children.push(new_node(
            "nvicPrioBits",
            format!("{}", self.nvic_priority_bits),
        ));
        children.push(new_node(
            "vendorSystickConfig",
            format!("{}", self.has_vendor_systick),
        ));

        if let Some(v) = &self.device_num_interrupts {
            children.push(new_node("deviceNumInterrupts", format!("{}", v)));
        }
        if let Some(v) = &self.sau_num_regions {
            children.push(new_node("sauNumRegions", format!("{}", v)));
        }

        let mut elem = Element::new("cpu");
        elem.children = children;
        Ok(elem)
    }
}
