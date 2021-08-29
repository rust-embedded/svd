use super::{new_node, Element, Encode, EncodeError};
use crate::svd::DimElement;

impl Encode for DimElement {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut e = Element::new("dimElement");

        e.children.push(new_node("dim", format!("{}", self.dim)));
        e.children.push(new_node(
            "dimIncrement",
            format!("0x{:X}", self.dim_increment),
        ));

        if let Some(di) = &self.dim_index {
            e.children.push(new_node("dimIndex", di.join(",")));
        }

        Ok(e)
    }
}
