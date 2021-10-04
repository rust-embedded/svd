use super::{new_node, Element, Encode, EncodeError};

use crate::svd::Cpu;
impl Encode for Cpu {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let children = vec![
            new_node("name", self.name.clone()),
            new_node("revision", self.revision.clone()),
            self.endian.encode_node()?,
            new_node("mpuPresent", format!("{}", self.mpu_present)),
            new_node("fpuPresent", format!("{}", self.fpu_present)),
            new_node("nvicPrioBits", format!("{}", self.nvic_priority_bits)),
            new_node(
                "vendorSystickConfig",
                format!("{}", self.has_vendor_systick),
            ),
        ];
        let mut elem = Element::new("cpu");
        elem.children = children;
        Ok(elem)
    }
}
