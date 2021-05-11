use super::Element;

use crate::types::Parse;

use crate::error::*;
use crate::svd::{DimElement, Register, RegisterInfo};

impl Parse for Register {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        assert_eq!(tree.name, "register");

        let info = RegisterInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree)?;
            check_has_placeholder(&info.name, "register")?;
            if let Some(indices) = &array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Ok(Register::Array(info, array_info))
        } else {
            Ok(Register::Single(info))
        }
    }
}
