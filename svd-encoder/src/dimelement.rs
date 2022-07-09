use crate::config::{change_case, format_number};

use super::{new_node, Config, Element, Encode, EncodeError};

impl Encode for crate::svd::DimElement {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut e = Element::new("dimElement");

        e.children
            .push(new_node("dim", format_number(self.dim, config.dim_dim)));
        e.children.push(new_node(
            "dimIncrement",
            format_number(self.dim_increment, config.dim_increment),
        ));

        if let Some(di) = &self.dim_index {
            e.children
                .push(if let Some(range) = self.indexes_as_range() {
                    new_node("dimIndex", format!("{}-{}", range.start(), range.end()))
                } else {
                    new_node("dimIndex", di.join(","))
                });
        }

        if let Some(dim_name) = &self.dim_name {
            e.children.push(new_node("dimName", dim_name.clone()))
        }

        if let Some(v) = &self.dim_array_index {
            e.children.push(v.encode_node_with_config(config)?);
        }

        Ok(e)
    }
}

impl Encode for crate::svd::DimArrayIndex {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut base = Element::new("dimArrayIndex");

        if let Some(d) = &self.header_enum_name {
            base.children.push(new_node(
                "headerEnumName",
                change_case(d, config.dim_array_index_header_enum_name),
            ));
        }

        for v in &self.values {
            base.children.push(v.encode_node_with_config(config)?);
        }

        Ok(base)
    }
}
