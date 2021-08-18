use super::{new_element, Element, Encode, EncodeChildren, EncodeError};

use crate::svd::RegisterProperties;

impl EncodeChildren for RegisterProperties {
    type Error = EncodeError;

    fn encode(&self) -> Result<Vec<Element>, EncodeError> {
        let mut children = Vec::new();

        if let Some(v) = &self.size {
            children.push(new_element("size", Some(format!("0x{:x}", v))));
        };

        if let Some(v) = &self.access {
            children.push(v.encode()?);
        };

        if let Some(v) = &self.reset_value {
            children.push(new_element(
                "resetValue",
                Some(if *v > std::u32::MAX as u64 {
                    format!("0x{:016X}", v)
                } else {
                    format!("0x{:08X}", v)
                }),
            ));
        };

        if let Some(v) = &self.reset_mask {
            children.push(new_element(
                "resetMask",
                Some(if *v > std::u32::MAX as u64 {
                    format!("0x{:016X}", v)
                } else {
                    format!("0x{:08X}", v)
                }),
            ));
        };

        Ok(children)
    }
}
