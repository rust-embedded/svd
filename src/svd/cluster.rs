use std::ops::Deref;
use xmltree::Element;

use svd::defaults::Defaults;
use parse::{Parse, ParseDefaults};

#[cfg(feature = "unproven")]
use elementext::ElementExt;
#[cfg(feature = "unproven")]
use encode::Encode;
use error::*;
use svd::clusterinfo::ClusterInfo;
use svd::registerclusterarrayinfo::RegisterClusterArrayInfo;

#[derive(Clone, Debug, PartialEq)]
pub enum Cluster {
    Single(ClusterInfo),
    Array(ClusterInfo, RegisterClusterArrayInfo),
}

impl Deref for Cluster {
    type Target = ClusterInfo;

    fn deref(&self) -> &ClusterInfo {
        match *self {
            Cluster::Single(ref info) => info,
            Cluster::Array(ref info, _) => info,
        }
    }
}

impl ParseDefaults for Cluster {
    type Object = Cluster;
    type Error = SVDError;
    fn parse_defaults(tree: &Element, defaults: Defaults) -> Result<Cluster, SVDError> {
        assert_eq!(tree.name, "cluster");

        let info = ClusterInfo::parse_defaults(tree, defaults)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterClusterArrayInfo::parse(tree)?;
            if !info.name.contains("%s") {
                // TODO: replace with real error
                return Err(SVDError::from(SVDErrorKind::Other(
                    "Cluster name invalid".to_string(),
                )));
            }

            if let Some(ref indices) = array_info.dim_index {
                if array_info.dim as usize != indices.len() {
                    // TODO: replace with real error
                    return Err(SVDError::from(SVDErrorKind::Other(
                        "Cluster index length mismatch".to_string(),
                    )));
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
    type Error = SVDError;
    // TODO: support Cluster encoding
    fn encode(&self) -> Result<Element, SVDError> {
        match self {
            Cluster::Single(i) => {
                let mut e = i.encode()?;
                Ok(e)
            }
            Cluster::Array(i, a) => {
                let mut e = i.encode()?;
                e = e.merge(&a.encode()?);
                Ok(e)
            }
        }
    }
}

// TODO: test Cluster encoding and decoding
