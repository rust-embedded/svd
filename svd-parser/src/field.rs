use super::{check_has_placeholder, Node, Parse, Result, SVDError};
use crate::elementext::ElementExt;
use crate::svd::{DimElement, Field, FieldInfo};

impl Parse for Field {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        if !tree.has_tag_name("field") {
            return Err(SVDError::NotExpectedTag("field".to_string())
                .at(tree.id())
                .into());
        }

        let info = FieldInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree)?;
            check_has_placeholder(&info.name, "field")?;
            if let Some(indexes) = &array_info.dim_index {
                if array_info.dim as usize != indexes.len() {
                    return Err(SVDError::IncorrectDimIndexesCount(
                        array_info.dim as usize,
                        indexes.len(),
                    )
                    .at(tree.id())
                    .into());
                }
            }
            Ok(Field::Array(info, array_info))
        } else {
            Ok(Field::Single(info))
        }
    }
}
