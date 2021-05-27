use super::{check_has_placeholder, Element, Parse, Result};
use crate::elementext::ElementExt;
use crate::svd::{DimElement, Field, FieldInfo};

impl Parse for Field {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        assert!(tree.has_tag_name("field"));

        let info = FieldInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree)?;
            check_has_placeholder(&info.name, "field")?;
            if let Some(indices) = &array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Ok(Field::Array(info, array_info))
        } else {
            Ok(Field::Single(info))
        }
    }
}
