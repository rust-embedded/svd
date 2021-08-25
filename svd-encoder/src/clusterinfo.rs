use super::{new_element, Element, Encode, EncodeChildren, EncodeError};

use crate::svd::ClusterInfo;

impl Encode for ClusterInfo {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut e = new_element("cluster", None);

        e.children
            .push(new_element("name", Some(self.name.clone())));

        if let Some(v) = &self.description {
            e.children.push(new_element("description", Some(v.clone())));
        }

        if let Some(v) = &self.header_struct_name {
            e.children
                .push(new_element("headerStructName", Some(v.clone())));
        }

        e.children.push(new_element(
            "addressOffset",
            Some(format!("{}", self.address_offset)),
        ));

        e.children
            .extend(self.default_register_properties.encode()?);

        for c in &self.children {
            e.children.push(c.encode()?);
        }

        if let Some(v) = &self.derived_from {
            e.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(e)
    }
}
