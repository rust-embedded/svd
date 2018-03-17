use std::collections::HashMap;

use xmltree::Element;
use failure::ResultExt;

use ElementExt;

use parse;
use types::{Parse, Encode, new_element};
use error::*;


#[derive(Clone, Debug, PartialEq)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: u32,
}

impl Interrupt {
    fn _parse(tree: &Element, name: String) -> Result<Interrupt, SVDError> {
        Ok(Interrupt {
            name,
            description: tree.get_child_text_opt("description")?,
            value: parse::get_child_u32("value", tree)?,
        })
    }
}

impl Parse for Interrupt {
    type Object = Interrupt;
    type Error = SVDError;
    fn parse(tree: &Element) -> Result<Interrupt, SVDError> {
        if tree.name != "interrupt" {
            return Err(SVDErrorKind::NotExpectedTag(tree.clone(), format!("interrupt")).into());
        }
        let name = tree.get_child_text("name")?;
        Interrupt::_parse(tree,name.clone()).context(SVDErrorKind::Other(format!("In interrupt `{}`", name))).map_err(|e| e.into())
    }
}



impl Encode for Interrupt {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        Ok(Element {
            name: String::from("interrupt"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("description", self.description.clone()),
                new_element("value", Some(format!("{}", self.value))),
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
        let types = vec![
            (
                Interrupt {
                    name: String::from("test"),
                    description: Some(String::from("description")),
                    value: 14,
                },
                String::from("
                <interrupt>
                    <name>test</name>
                    <description>description</description>
                    <value>14</value>
                </interrupt>",
                )
            ),
        ];

        for (a, s) in types {
            let tree1 = Element::parse(s.as_bytes()).unwrap();
            let v = Interrupt::parse(&tree1).unwrap();
            assert_eq!(v, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = v.encode().unwrap();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
