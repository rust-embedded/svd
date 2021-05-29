use super::{check_has_placeholder, Config, Node, Parse, Result, SVDError};
use crate::elementext::ElementExt;
use crate::svd::{Cluster, ClusterInfo, DimElement};

impl Parse for Cluster {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self> {
        if !tree.has_tag_name("cluster") {
            return Err(SVDError::NotExpectedTag("cluster".to_string())
                .at(tree.id())
                .into());
        }

        let info = ClusterInfo::parse(tree, config)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree, config)?;
            check_has_placeholder(&info.name, "cluster")?;

            if let Some(indices) = &array_info.dim_index {
                if array_info.dim as usize != indices.len() {
                    // TODO: replace with real error
                    anyhow::bail!("Cluster index length mismatch");
                }
            }

            Ok(Cluster::Array(info, array_info))
        } else {
            Ok(Cluster::Single(info))
        }
    }
}
