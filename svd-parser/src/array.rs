use super::*;
use svd_rs::{DimElement, Name, SingleArray};

pub fn parse_array<T>(tag: &str, tree: &Node, config: &Config) -> Result<SingleArray<T>, SVDErrorAt>
where
    T: Parse<Object = T, Error = SVDErrorAt, Config = Config> + Name,
{
    if !tree.has_tag_name(tag) {
        return Err(SVDError::NotExpectedTag(tag.into()).at(tree.id()));
    }

    let info = T::parse(tree, config)?;

    if tree.get_child("dimIncrement").is_some() {
        let array_info = DimElement::parse(tree, config)?;
        check_has_placeholder(info.name(), tag).map_err(|e| e.at(tree.id()))?;
        if let Some(indexes) = &array_info.dim_index {
            if array_info.dim as usize != indexes.len() {
                return Err(SVDError::IncorrectDimIndexesCount(
                    array_info.dim as usize,
                    indexes.len(),
                )
                .at(tree.id()));
            }
        }
        Ok(SingleArray::Array(info, array_info))
    } else {
        Ok(SingleArray::Single(info))
    }
}
