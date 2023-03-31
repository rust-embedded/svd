use super::{new_node, Config, Element, Encode, EncodeChildren, EncodeError, XMLNode};
use crate::{config::Sorting, svd::Device};

impl Encode for Device {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("device");
        if let Some(v) = &self.vendor {
            elem.children.push(new_node("vendor", v.clone()));
        }
        if let Some(v) = &self.vendor_id {
            elem.children.push(new_node("vendorID", v.clone()));
        }

        elem.children.push(new_node("name", self.name.clone()));

        if let Some(v) = &self.series {
            elem.children.push(new_node("series", v.clone()));
        }

        elem.children
            .push(new_node("version", self.version.clone()));

        elem.children
            .push(new_node("description", self.description.clone()));

        if let Some(v) = &self.license_text {
            elem.children.push(new_node("licenseText", v.clone()));
        }

        if let Some(v) = &self.cpu {
            elem.children
                .push(XMLNode::Element(v.encode_with_config(config)?));
        }

        if let Some(v) = &self.header_system_filename {
            elem.children
                .push(new_node("headerSystemFilename", v.clone()));
        }

        if let Some(v) = &self.header_definitions_prefix {
            elem.children
                .push(new_node("headerDefinitionsPrefix", v.clone()));
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

        let peripherals: Result<Vec<_>, _> = if let Some(sorting) = config.peripheral_sorting {
            let mut refs = self.peripherals.iter().collect::<Vec<_>>();
            match sorting {
                Sorting::Offset => refs.sort_by_key(|p| p.base_address),
                Sorting::OffsetReversed => {
                    refs.sort_by_key(|p| -(p.base_address as i32));
                }
                Sorting::Name => refs.sort_by_key(|p| &p.name),
            }
            refs.into_iter()
                .map(|peripheral| peripheral.encode_node_with_config(config))
                .collect()
        } else {
            self.peripherals
                .iter()
                .map(|peripheral| peripheral.encode_node_with_config(config))
                .collect()
        };
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
