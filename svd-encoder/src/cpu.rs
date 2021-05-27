use super::{new_element, Element, Encode, EncodeError};

use crate::svd::Cpu;
impl Encode for Cpu {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let children = vec![
            new_element("name", Some(self.name.clone())),
            new_element("revision", Some(self.revision.clone())),
            self.endian.encode()?,
            new_element("mpuPresent", Some(format!("{}", self.mpu_present))),
            new_element("fpuPresent", Some(format!("{}", self.fpu_present))),
            new_element("nvicPrioBits", Some(format!("{}", self.nvic_priority_bits))),
            new_element(
                "vendorSystickConfig",
                Some(format!("{}", self.has_vendor_systick)),
            ),
        ];
        let mut elem = new_element("cpu", None);
        elem.children = children;
        Ok(elem)
    }
}
