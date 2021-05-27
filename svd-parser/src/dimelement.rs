use super::{
    elementext::ElementExt, optional, types::DimIndex, Config, Node, Parse, SVDError, SVDErrorAt,
};
use crate::svd::DimElement;

impl Parse for DimElement {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        DimElement::builder()
            .dim(tree.get_child_u32("dim")?)
            .dim_increment(tree.get_child_u32("dimIncrement")?)
            .dim_index(optional::<DimIndex>("dimIndex", tree, config)?)
            .build()
            .map_err(|e| SVDError::from(e).at(tree.id()).into())
    }
}
