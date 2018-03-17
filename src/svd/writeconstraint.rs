
use std::collections::HashMap;

use xmltree::Element;

use ::parse;
use ::types::{Parse, Encode, new_element};
use ::error::SVDError;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WriteConstraint {
    WriteAsRead(bool),
    UseEnumeratedValues(bool),
    Range(WriteConstraintRange),
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WriteConstraintRange {
    pub min: u32,
    pub max: u32,
}

impl Parse for WriteConstraint {
    type Object = WriteConstraint;
    type Error = SVDError;
    
    fn parse(tree: &Element) -> Result<WriteConstraint, SVDError> {
        if tree.children.len() == 1 {
            let ref field = tree.children[0].name;
            // Write constraint can only be one of the following
            match field.as_ref() {
                "writeAsRead" => {
                    Ok(WriteConstraint::WriteAsRead(parse::get_child_bool(field.as_ref(), tree)?))
                }
                "useEnumeratedValues" => {
                    Ok(WriteConstraint::UseEnumeratedValues(parse::get_child_bool(field.as_ref(), tree)?))
                }
                "range" => {
                    Ok(WriteConstraint::Range(WriteConstraintRange::parse(parse::get_child_elem(field.as_ref(), tree)?)?))
                }
                _ => Err(SVDError::UnknownWriteConstraint(tree.clone())),
            }
        } else {
            Err(SVDError::MoreThanOneWriteConstraint(tree.clone()))
        }
    }
}

impl Encode for WriteConstraint {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let v = match *self {
            WriteConstraint::WriteAsRead(v) => new_element("writeAsRead", Some(format!("{}", v))),
            WriteConstraint::UseEnumeratedValues(v) => new_element("useEnumeratedValues", Some(format!("{}", v))),
            WriteConstraint::Range(v) => v.encode()?,
        };

        Ok(Element {
            name: String::from("WriteConstraint"),
            attributes: HashMap::new(),
            children: vec![v],
            text: None,
        })
    }
}

impl Parse for WriteConstraintRange {
    type Object = WriteConstraintRange;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<WriteConstraintRange, SVDError> {
        Ok(WriteConstraintRange {  
            min: parse::get_child_u32("minimum", tree)?,
            max: parse::get_child_u32("maximum", tree)?,
        })
    }
}

impl Encode for WriteConstraintRange {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        Ok(Element {
            name: String::from("range"),
            attributes: HashMap::new(),
            children: vec![
                new_element("minimum", Some(format!("0x{:08.x}", self.min))),
                new_element("maximum", Some(format!("0x{:08.x}", self.max))),
            ],
            text: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let examples = vec![
            (
                String::from(
                    "<WriteConstraint><writeAsRead>true</writeAsRead></WriteConstraint>",
                ),
                WriteConstraint::WriteAsRead(true)
            ),
            (
                String::from(
                    "<WriteConstraint><useEnumeratedValues>true</useEnumeratedValues></WriteConstraint>",
                ),
                WriteConstraint::UseEnumeratedValues(true)
            ),
            (
                String::from(
                    "<WriteConstraint><range><minimum>0x00000001</minimum><maximum>0x0000000a</maximum></range></WriteConstraint>",
                ),
                WriteConstraint::Range(WriteConstraintRange{min: 1, max: 10})
            ),
        ];

        for (example, expected) in examples {
            let tree1 = Element::parse(example.as_bytes()).unwrap();

            let parsed = WriteConstraint::parse(&tree1).unwrap();
            assert_eq!(parsed, expected, "Parsing tree failed");

            let tree2 = parsed.encode().unwrap();
            assert_eq!(tree1, tree2, "Encoding value failed");
        }

    }
}
