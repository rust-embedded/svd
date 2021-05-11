use super::{new_element, Element, Encode};
use crate::error::*;
use crate::svd::{WriteConstraint, WriteConstraintRange};

impl Encode for WriteConstraint {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let v = match *self {
            WriteConstraint::WriteAsRead(v) => new_element("writeAsRead", Some(format!("{}", v))),
            WriteConstraint::UseEnumeratedValues(v) => {
                new_element("useEnumeratedValues", Some(format!("{}", v)))
            }
            WriteConstraint::Range(v) => v.encode()?,
        };

        let mut elem = new_element("writeConstraint", None);
        elem.children = vec![v];
        Ok(elem)
    }
}

impl Encode for WriteConstraintRange {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut elem = new_element("range", None);
        elem.children = vec![
            new_element("minimum", Some(format!("{}", self.min))),
            new_element("maximum", Some(format!("{}", self.max))),
        ];
        Ok(elem)
    }
}
