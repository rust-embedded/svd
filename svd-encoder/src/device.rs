use super::{new_node, Config, Element, Encode, EncodeChildren, EncodeError, XMLNode};
use crate::svd::Device;

impl Encode for Device {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("device");
        if let Some(v) = &self.vendor {
            elem.children.push(new_node("vendor", v));
        }
        if let Some(v) = &self.vendor_id {
            elem.children.push(new_node("vendorID", v));
        }

        elem.children.push(new_node("name", self.name));

        if let Some(v) = &self.series {
            elem.children.push(new_node("series", v));
        }

        elem.children.push(new_node("version", self.version));

        elem.children
            .push(new_node("description", self.description));

        if let Some(v) = &self.license_text {
            elem.children.push(new_node("licenseText", v));
        }

        if let Some(v) = &self.cpu {
            elem.children
                .push(XMLNode::Element(v.encode_with_config(config)?));
        }

        if let Some(v) = &self.header_system_filename {
            elem.children.push(new_node("headerSystemFilename", v));
        }

        if let Some(v) = &self.header_definitions_prefix {
            elem.children.push(new_node("header_definitions_prefix", v));
        }

        elem.children.push(new_node(
            "addressUnitBits",
            format!("{}", self.address_unit_bits),
        ));

        elem.children
            .push(new_node("width", format!("{}", self.width)));

        elem.children.extend(
            self.default_register_properties
                .encode_with_config(config)?,
        );

        let peripherals: Result<Vec<_>, _> = self
            .peripherals
            .iter()
            .map(|peripheral| peripheral.encode_node_with_config(config))
            .collect();
        elem.children.push({
            let mut e = Element::new("peripherals");
            e.children = peripherals?;
            XMLNode::Element(e)
        });

        elem.attributes
            .insert(String::from("schemaVersion"), self.schema_version.clone());
        elem.attributes
            .insert(String::from("xmlns:xs"), self.xmlns_xs.clone());
        elem.attributes.insert(
            String::from("xs:noNamespaceSchemaLocation"),
            self.no_namespace_schema_location.clone(),
        );

        Ok(elem)
    }
}
