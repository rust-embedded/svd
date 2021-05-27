use super::{check_has_placeholder, Element, Parse, Result};
use crate::elementext::ElementExt;
use crate::svd::{Cluster, ClusterInfo, DimElement};

impl Parse for Cluster {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        assert!(tree.has_tag_name("cluster"));

        let info = ClusterInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree)?;
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
