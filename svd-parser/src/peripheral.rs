use super::*;
use crate::elementext::ElementExt;
use crate::svd::{DimElement, Peripheral, PeripheralInfo};

impl Parse for Peripheral {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("peripheral") {
            return Err(SVDError::NotExpectedTag("peripheral".to_string()).at(tree.id()));
        }

        let info = PeripheralInfo::parse(tree, config)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree, config)?;
            check_has_placeholder(&info.name, "peripheral").map_err(|e| e.at(tree.id()))?;
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
