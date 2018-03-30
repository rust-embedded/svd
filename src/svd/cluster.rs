
use std::ops::Deref;
use xmltree::Element;

use types::{Parse, Encode};
use error::*;
use ::svd::clusterinfo::ClusterInfo;
use ::svd::registerclusterarrayinfo::RegisterClusterArrayInfo;

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

impl Parse for Cluster {
    type Object = Cluster;
    type Error = SVDError;
    fn parse(tree: &Element) -> Result<Cluster, SVDError> {
        assert_eq!(tree.name, "cluster");

        let info = ClusterInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            // TODO: s/assert/errors/g
            let array_info = RegisterClusterArrayInfo::parse(tree)?;
            assert!(info.name.contains("%s"));

            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }

            Ok(Cluster::Array(info, array_info))
        } else {
            Ok(Cluster::Single(info))
        }
    }
}

impl Encode for Cluster {
    type Error = SVDError;
    // TODO: encoding here
    fn encode(&self) -> Result<Element, SVDError> {
        Err(SVDError::from(SVDErrorKind::EncodeNotImplemented(String::from("RegisterClusterArrayInfo"))))
    }
}