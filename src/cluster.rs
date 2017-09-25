
use std::ops::Deref;
use std::collections::hash_map::*;

use xmltree::Element;


use helpers::*;
use clusterinfo::*;
use registerclusterarrayinfo::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Cluster {
    Single(ClusterInfo),
    Array(ClusterInfo, RegisterClusterArrayInfo),
}

impl ParseElem for Cluster {
    fn parse(tree: &Element) -> Cluster {
        assert_eq!(tree.name, "cluster");

        let info = ClusterInfo::parse(tree);

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterClusterArrayInfo::parse(tree);
            assert!(info.name.contains("%s"));
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Cluster::Array(info, array_info)
        } else {
            Cluster::Single(info)
        }
    }
}

impl EncodeElem for Cluster {
    fn encode(&self) -> Element {
        Element {
            name: String::from("cluster"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: None,
        }
    }
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
