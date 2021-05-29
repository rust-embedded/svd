use super::*;
use crate::svd::{WriteConstraint, WriteConstraintRange};

impl Parse for WriteConstraint {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self, Self::Error> {
        let child = tree.first_element_child().unwrap();
        if child.next_sibling_element().is_some() {
            return Err(SVDError::MoreThanOneWriteConstraint.at(tree.id()));
        }
        let field = child.tag_name().name();
        // Write constraint can only be one of the following
        match field {
            "writeAsRead" => tree.get_child_bool(field).map(WriteConstraint::WriteAsRead),
            "useEnumeratedValues" => tree
                .get_child_bool(field)
                .map(WriteConstraint::UseEnumeratedValues),
            "range" => WriteConstraintRange::parse(&tree.get_child_elem(field)?, &())
                .map(WriteConstraint::Range),
            _ => Err(SVDError::UnknownWriteConstraint.at(tree.id())),
        }
    }
}

impl Parse for WriteConstraintRange {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = ();

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self, Self::Error> {
        Ok(Self {
            min: tree.get_child_u64("minimum")?,
            max: tree.get_child_u64("maximum")?,
        })
    }
}
