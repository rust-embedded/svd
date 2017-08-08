extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;

pub trait ParseElem {
    fn parse(tree: &Element) -> Self;
}

pub trait ParseOption {
    fn parse(tree: &Element) -> Self;
}

pub trait EncodeElem {
    fn encode(&self) -> Element;
}

pub fn new_element(name: &str, text: Option<String>) -> Element {
    Element{
        name: String::from(name),
        attributes: HashMap::new(),
        children: Vec::new(),
        text: text,
    }
}