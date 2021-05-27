use super::{new_element, Element, Encode, EncodeError};
use crate::svd::DimElement;

impl Encode for DimElement {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut e = new_element("dimElement", None);

        e.children
            .push(new_element("dim", Some(format!("{}", self.dim))));
        e.children.push(new_element(
            "dimIncrement",
            Some(format!("0x{:X}", self.dim_increment)),
        ));

        if let Some(di) = &self.dim_index {
            e.children.push(new_element("dimIndex", Some(di.join(","))));
        }

        Ok(e)
    }
}
