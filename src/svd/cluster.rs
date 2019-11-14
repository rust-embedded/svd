use core::ops::Deref;
use xmltree::Element;

use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
use crate::svd::{clusterinfo::ClusterInfo, dimelement::DimElement};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Cluster {
    Single(ClusterInfo),
    Array(ClusterInfo, DimElement),
}

impl Deref for Cluster {
    type Target = ClusterInfo;

    fn deref(&self) -> &ClusterInfo {
        match self {
            Cluster::Single(info) => info,
            Cluster::Array(info, _) => info,
        }
    }
}

impl Parse for Cluster {
    type Object = Cluster;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Cluster> {
        assert_eq!(tree.name, "cluster");

        let info = ClusterInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree)?;
            if !info.name.contains("%s") {
                // TODO: replace with real error
                anyhow::bail!("Cluster name invalid");
            }

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

#[cfg(feature = "unproven")]
impl Encode for Cluster {
    type Error = anyhow::Error;

    // TODO: support Cluster encoding
    fn encode(&self) -> Result<Element> {
        match self {
            Cluster::Single(i) => i.encode(),
            Cluster::Array(i, a) => {
                let mut e = i.encode()?;
                e = e.merge(&a.encode()?);
                Ok(e)
            }
        }
    }
}

// TODO: test Cluster encoding and decoding
