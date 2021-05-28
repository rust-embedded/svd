use super::{check_has_placeholder, Node, Parse, Result, SVDError};
use crate::elementext::ElementExt;
use crate::svd::{DimElement, Register, RegisterInfo};

impl Parse for Register {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        if !tree.has_tag_name("register") {
            return Err(SVDError::NotExpectedTag("register".to_string())
                .at(tree.id())
                .into());
        }

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
