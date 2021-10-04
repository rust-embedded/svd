use super::{new_node, Element, Encode, EncodeError};

use crate::svd::{WriteConstraint, WriteConstraintRange};

impl Encode for WriteConstraint {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let v = match *self {
            WriteConstraint::WriteAsRead(v) => new_node("writeAsRead", format!("{}", v)),
            WriteConstraint::UseEnumeratedValues(v) => {
                new_node("useEnumeratedValues", format!("{}", v))
            }
            WriteConstraint::Range(v) => v.encode_node()?,
        };

        let mut elem = Element::new("writeConstraint");
        elem.children = vec![v];
        Ok(elem)
    }
}

impl Encode for WriteConstraintRange {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("range");
        elem.children = vec![
            new_node("minimum", format!("{}", self.min)),
            new_node("maximum", format!("{}", self.max)),
        ];
        Ok(elem)
    }
}
