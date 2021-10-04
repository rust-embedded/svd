use super::{new_node, Element, Encode, EncodeChildren, EncodeError, XMLNode};

use crate::svd::ClusterInfo;

impl Encode for ClusterInfo {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut e = Element::new("cluster");

        e.children.push(new_node("name", self.name.clone()));

        if let Some(v) = &self.description {
            e.children.push(new_node("description", v.clone()));
        }

        if let Some(v) = &self.header_struct_name {
            e.children.push(new_node("headerStructName", v.clone()));
        }

        e.children.push(new_node(
            "addressOffset",
            format!("{}", self.address_offset),
        ));

        e.children
            .extend(self.default_register_properties.encode()?);

        for c in &self.children {
            e.children.push(XMLNode::Element(c.encode()?));
        }

        if let Some(v) = &self.derived_from {
            e.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(e)
    }
}
