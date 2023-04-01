use svd_rs::Field;

use super::{
    new_node, Config, Element, ElementMerge, Encode, EncodeChildren, EncodeError, XMLNode,
};

use crate::{
    config::{change_case, format_number, Sorting},
    svd::{Register, RegisterInfo},
};

impl Encode for Register {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        match self {
            Self::Single(info) => info.encode_with_config(config),
            Self::Array(info, array_info) => {
                let mut base = Element::new("register");
                base.merge(&array_info.encode_with_config(config)?);
                base.merge(&info.encode_with_config(config)?);
                Ok(base)
            }
        }
    }
}

impl Encode for RegisterInfo {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("register");
        elem.children.push(new_node(
            "name",
            change_case(&self.name, config.register_name),
        ));

        if let Some(v) = &self.display_name {
            elem.children.push(new_node("displayName", v.clone()));
        }

        if let Some(v) = &self.description {
            elem.children.push(new_node("description", v.clone()));
        }

        if let Some(v) = &self.alternate_group {
            elem.children
                .push(new_node("alternateGroup", v.to_string()));
        }

        if let Some(v) = &self.alternate_register {
            elem.children.push(new_node(
                "alternateRegister",
                change_case(v, config.register_name),
            ));
        }

        elem.children.push(new_node(
            "addressOffset",
            format_number(self.address_offset, config.register_address_offset),
        ));

        elem.children
            .extend(self.properties.encode_with_config(config)?);

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode_node_with_config(config)?);
        }

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.read_action {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.fields {
            let children: Result<Vec<_>, _> =
                if !config.field_derived_last && config.field_sorting.is_none() {
                    v.iter()
                        .map(|field| field.encode_node_with_config(config))
                        .collect()
                } else {
                    sort_derived_fields(v, config.field_sorting, config.field_derived_last)
                        .into_iter()
                        .map(|field| field.encode_node_with_config(config))
                        .collect()
                };

            let children = children?;
            if !children.is_empty() {
                let mut fields = Element::new("fields");
                fields.children = children;
                elem.children.push(XMLNode::Element(fields));
            }
        }

        if let Some(v) = &self.derived_from {
            elem.attributes.insert(
                String::from("derivedFrom"),
                change_case(v, config.register_name),
            );
        }

        Ok(elem)
    }
}

fn sort_fields(refs: &mut [&Field], sorting: Option<Sorting>) {
    if let Some(sorting) = sorting {
        match sorting {
            Sorting::Offset => refs.sort_by_key(|f| f.bit_offset()),
            Sorting::OffsetReversed => {
                refs.sort_by_key(|f| -(f.bit_offset() as i32));
            }
            Sorting::Name => refs.sort_by_key(|f| &f.name),
        }
    }
}

fn sort_derived_fields(v: &[Field], sorting: Option<Sorting>, derived_last: bool) -> Vec<&Field> {
    if derived_last {
        let mut common_refs = Vec::with_capacity(v.len());
        let mut derived_refs = Vec::new();
        for f in v.iter() {
            if f.derived_from.is_some() {
                derived_refs.push(f);
            } else {
                let mut derived = false;
                for ev in &f.enumerated_values {
                    if ev.derived_from.is_some() {
                        derived_refs.push(f);
                        derived = true;
                        break;
                    }
                }
                if !derived {
                    common_refs.push(f);
                }
            }
        }
        sort_fields(&mut common_refs, sorting);
        sort_fields(&mut derived_refs, sorting);
        common_refs.extend(derived_refs);

        common_refs
    } else {
        let mut refs = v.iter().collect::<Vec<_>>();
        sort_fields(&mut refs, sorting);
        refs
    }
}
