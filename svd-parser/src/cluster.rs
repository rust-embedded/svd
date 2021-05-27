use super::{check_has_placeholder, Config, Node, Parse, SVDError, SVDErrorAt};
use crate::elementext::ElementExt;
use crate::svd::{Cluster, ClusterInfo, DimElement};

impl Parse for Cluster {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("cluster") {
            return Err(SVDError::NotExpectedTag("cluster".to_string())
                .at(tree.id())
                .into());
        }

        let info = ClusterInfo::parse(tree, config)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree, config)?;
            check_has_placeholder(&info.name, "cluster").map_err(|e| e.at(tree.id()))?;
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

            Ok(Cluster::Array(info, array_info))
        } else {
            Ok(Cluster::Single(info))
        }
    }
}
