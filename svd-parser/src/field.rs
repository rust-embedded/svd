use super::*;
use crate::svd::{DimElement, Field, FieldInfo};
use crate::{elementext::ElementExt, Config};

impl Parse for Field {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("field") {
            return Err(SVDError::NotExpectedTag("field".to_string()).at(tree.id()));
        }

        let info = FieldInfo::parse(tree, config)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree, config)?;
            check_has_placeholder(&info.name, "field").map_err(|e| e.at(tree.id()))?;
            if let Some(indexes) = &array_info.dim_index {
                if array_info.dim as usize != indexes.len() {
                    return Err(SVDError::IncorrectDimIndexesCount(
                        array_info.dim as usize,
                        indexes.len(),
                    )
                    .at(tree.id()));
                }
            }
            Ok(info.array(array_info))
        } else {
            Ok(info.single())
        }
    }
}
