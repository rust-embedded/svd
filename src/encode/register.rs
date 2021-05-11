use super::{new_element, Element, Encode};

use crate::elementext::ElementExt;
use crate::error::*;

use crate::svd::Register;
impl Encode for Register {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        match self {
            Register::Single(info) => info.encode(),
            Register::Array(info, array_info) => {
                let mut base = new_element("register", None);
                base.merge(&array_info.encode()?);
                base.merge(&info.encode()?);
                Ok(base)
            }
        }
    }
}
