use super::{
    elementext::ElementExt,
    types::{parse_optional, DimIndex},
    Element, Parse,
};
use crate::error::*;
use crate::svd::DimElement;

impl Parse for DimElement {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        Ok(DimElement::builder()
            .dim(tree.get_child_u32("dim")?)
            .dim_increment(tree.get_child_u32("dimIncrement")?)
            .dim_index(parse_optional::<DimIndex>("dimIndex", tree)?)
            .build()?)
    }
}
