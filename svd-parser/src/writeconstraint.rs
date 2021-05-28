use super::{elementext::ElementExt, Node, Parse, Result, SVDError};
use crate::svd::{WriteConstraint, WriteConstraintRange};

impl Parse for WriteConstraint {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        let child = tree.first_element_child().unwrap();
        if child.next_sibling_element().is_some() {
            return Err(SVDError::MoreThanOneWriteConstraint(tree.id()).into());
        }
        let field = child.tag_name().name();
        // Write constraint can only be one of the following
        match field {
            "writeAsRead" => Ok(WriteConstraint::WriteAsRead(tree.get_child_bool(field)?)),
            "useEnumeratedValues" => Ok(WriteConstraint::UseEnumeratedValues(
                tree.get_child_bool(field)?,
            )),
            "range" => Ok(WriteConstraint::Range(WriteConstraintRange::parse(
                &tree.get_child_elem(field)?,
            )?)),
            _ => Err(SVDError::UnknownWriteConstraint(tree.id()).into()),
        }
    }
}

impl Parse for WriteConstraintRange {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        Ok(Self {
            min: tree.get_child_u64("minimum")?,
            max: tree.get_child_u64("maximum")?,
        })
    }
}
