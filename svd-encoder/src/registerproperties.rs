use super::{new_node, Encode, EncodeChildren, EncodeError, XMLNode};

use crate::svd::RegisterProperties;

impl EncodeChildren for RegisterProperties {
    type Error = EncodeError;

    fn encode(&self) -> Result<Vec<XMLNode>, EncodeError> {
        let mut children = Vec::new();

        if let Some(v) = &self.size {
            children.push(new_node("size", format!("0x{:x}", v)));
        };

        if let Some(v) = &self.access {
            children.push(v.encode_node()?);
        };

        if let Some(v) = &self.protection {
            children.push(v.encode_node()?);
        };

        if let Some(v) = &self.reset_value {
            children.push(new_node(
                "resetValue",
                if *v > std::u32::MAX as u64 {
                    format!("0x{:016X}", v)
                } else {
                    format!("0x{:08X}", v)
                },
            ));
        };

        if let Some(v) = &self.reset_mask {
            children.push(new_node(
                "resetMask",
                if *v > std::u32::MAX as u64 {
                    format!("0x{:016X}", v)
                } else {
                    format!("0x{:08X}", v)
                },
            ));
        };

        Ok(children)
    }
}
