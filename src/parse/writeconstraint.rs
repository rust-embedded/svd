use super::{elementext::ElementExt, Element, Parse};

use crate::error::*;
use crate::svd::{WriteConstraint, WriteConstraintRange};

impl Parse for WriteConstraint {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.children.len() == 1 {
            let field = &tree.children[0].name;
            // Write constraint can only be one of the following
            match field.as_ref() {
                "writeAsRead" => Ok(WriteConstraint::WriteAsRead(
                    tree.get_child_bool(field.as_ref())?,
                )),
                "useEnumeratedValues" => Ok(WriteConstraint::UseEnumeratedValues(
                    tree.get_child_bool(field.as_ref())?,
                )),
                "range" => Ok(WriteConstraint::Range(WriteConstraintRange::parse(
                    tree.get_child_elem(field.as_ref())?,
                )?)),
                _ => Err(SVDError::UnknownWriteConstraint(tree.clone()).into()),
            }
        } else {
            Err(SVDError::MoreThanOneWriteConstraint(tree.clone()).into())
        }
    }
}

impl Parse for WriteConstraintRange {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        Ok(Self {
            min: tree.get_child_u64("minimum")?,
            max: tree.get_child_u64("maximum")?,
        })
    }
}
