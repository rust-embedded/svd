
use std::collections::HashMap;

use xmltree::Element;

use parse;
use helpers::*;
use writeconstraintrange::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WriteConstraint {
    WriteAsRead(bool),
    UseEnumeratedValues(bool),
    Range(WriteConstraintRange),
}

impl ParseElem for WriteConstraint {
    fn parse(tree: &Element) -> WriteConstraint {
        if tree.children.len() == 1 {
            let ref field = tree.children[0].name;
            // Write constraint can only be one of the following
            match field.as_ref() {
                "writeAsRead" => {
                    WriteConstraint::WriteAsRead(try_get_child!(tree.get_child(field.as_ref()).map(|t| {
                        try_get_child!(parse::bool(t))
                    })))
                }
                "useEnumeratedValues" => {
                    WriteConstraint::UseEnumeratedValues(try_get_child!(tree.get_child(field.as_ref()).map(|t| {
                        try_get_child!(parse::bool(t))
                    })))
                }
                "range" => {
                    WriteConstraint::Range(try_get_child!(tree.get_child(field.as_ref()).map(
                        WriteConstraintRange::parse,
                    )))
                }
                v => panic!("unknown <writeConstraint> variant: {}", v),
            }
        } else {
            panic!("found more than one <WriteConstraint> element")
        }
    }
}

impl EncodeElem for WriteConstraint {
    fn encode(&self) -> Element {
        let v = match *self {
            WriteConstraint::WriteAsRead(v) => new_element("writeAsRead", Some(format!("{}", v))),
            WriteConstraint::UseEnumeratedValues(v) => new_element("useEnumeratedValues", Some(format!("{}", v))),
            WriteConstraint::Range(v) => v.encode(),
        };

        Element {
            name: String::from("WriteConstraint"),
            attributes: HashMap::new(),
            children: vec![v],
            text: None,
        }
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
        ];

        for (example, expected) in examples {
            let tree1 = &try_get_child!(Element::parse(example.as_bytes()));

            let parsed = WriteConstraint::parse(tree1);
            assert_eq!(parsed, expected, "Parsing tree failed");

            let tree2 = &parsed.encode();
            assert_eq!(tree1, tree2, "Encoding value failed");
        }

    }
}
