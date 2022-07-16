use super::{
    new_node, Config, Element, ElementMerge, Encode, EncodeChildren, EncodeError, XMLNode,
};

use crate::{
    config::{change_case, format_number},
    svd::{Cluster, ClusterInfo},
};

impl Encode for Cluster {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        match self {
            Self::Single(i) => i.encode_with_config(config),
            Self::Array(i, a) => {
                let mut e = Element::new("cluster");
                e.merge(&a.encode_with_config(config)?);
                e.merge(&i.encode_with_config(config)?);
                Ok(e)
            }
        }
    }
}

impl Encode for ClusterInfo {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut e = Element::new("cluster");

        e.children.push(new_node(
            "name",
            change_case(&self.name, config.cluster_name),
        ));

        if let Some(v) = &self.description {
            e.children.push(new_node("description", v));
        }

        if let Some(v) = &self.alternate_cluster {
            e.children.push(new_node(
                "alternateCluster",
                change_case(v, config.cluster_name),
            ));
        }

        if let Some(v) = &self.header_struct_name {
            e.children.push(new_node("headerStructName", v));
        }

        e.children.push(new_node(
            "addressOffset",
            format_number(self.address_offset, config.cluster_address_offset),
        ));

        e.children.extend(
            self.default_register_properties
                .encode_with_config(config)?,
        );

        for c in &self.children {
            e.children
                .push(XMLNode::Element(c.encode_with_config(config)?));
        }

        if let Some(v) = &self.derived_from {
            e.attributes.insert(
                String::from("derivedFrom"),
                change_case(v, config.cluster_name),
            );
        }

        Ok(e)
    }
}
