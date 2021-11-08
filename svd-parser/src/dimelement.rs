use super::types::DimIndex;
use super::*;
use crate::svd::{DimArrayIndex, DimElement, EnumeratedValue};

impl Parse for DimElement {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        DimElement::builder()
            .dim(tree.get_child_u32("dim")?)
            .dim_increment(tree.get_child_u32("dimIncrement")?)
            .dim_index(optional::<DimIndex>("dimIndex", tree, config)?)
            .dim_name(tree.get_child_text_opt("dimName")?)
            .dim_array_index(optional::<DimArrayIndex>("dimArrayIndex", tree, config)?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}

impl Parse for DimArrayIndex {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        Ok(Self {
            header_enum_name: tree.get_child_text_opt("headerEnumName")?,
            values: {
                let values: Result<Vec<_>, _> = tree
                    .children()
                    .filter(|t| t.is_element() && !matches!(t.tag_name().name(), "headerEnumName"))
                    .map(|t| {
                        if t.has_tag_name("enumeratedValue") {
                            EnumeratedValue::parse(&t, config)
                        } else {
                            Err(SVDError::NotExpectedTag("enumeratedValue".to_string()).at(t.id()))
                        }
                    })
                    .collect();
                values?
            },
        })
    }
}
