use super::{new_node, Element, Encode, EncodeChildren, EncodeError, XMLNode};
use crate::svd::{Device, Peripheral};

impl Encode for Device {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("device");
        elem.children.push(new_node("name", self.name.clone()));

        if let Some(v) = &self.version {
            elem.children.push(new_node("version", v.clone()));
        }

        if let Some(v) = &self.description {
            elem.children.push(new_node("description", v.clone()));
        }

        if let Some(v) = &self.cpu {
            elem.children.push(XMLNode::Element(v.encode()?));
        }

        if let Some(v) = &self.address_unit_bits {
            elem.children
                .push(new_node("addressUnitBits", format!("{}", v)));
        }

        if let Some(v) = &self.width {
            elem.children.push(new_node("width", format!("{}", v)));
        }

        elem.children
            .extend(self.default_register_properties.encode()?);

        let peripherals: Result<Vec<_>, _> = self
            .peripherals
            .iter()
            .map(Peripheral::encode_node)
            .collect();
        elem.children.push({
            let mut e = Element::new("peripherals");
            e.children = peripherals?;
            XMLNode::Element(e)
        });

        elem.attributes.insert(
            String::from("xmlns:xs"),
            String::from("http://www.w3.org/2001/XMLSchema-instance"),
        );
        if let Some(schema_version) = &self.schema_version {
            elem.attributes
                .insert(String::from("schemaVersion"), schema_version.to_string());
        }
        if let Some(schema_version) = &self.schema_version {
            elem.attributes.insert(
                String::from("xs:noNamespaceSchemaLocation"),
                format!("CMSIS-SVD_Schema_{}.xsd", schema_version.replace(".", "_")),
            );
        }

        Ok(elem)
    }
}
