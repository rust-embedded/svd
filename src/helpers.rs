extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;

// ParseElem parses an object from an XML element
pub trait ParseElem {
    fn parse(tree: &Element) -> Self;
}

// ParseChildren parses an object from children of an XML element
pub trait ParseChildren {
    fn parse_children(tree: &Element) -> Self;
}

// EncodeElem trait encodes an object to an XML element
pub trait EncodeElem {
    fn encode(&self) -> Element;
}

// EncodeChildren trait encodes an object to a vector of child elements
pub trait EncodeChildren {
    fn encode_children(&self) -> Vec<Element>;
}

// Helper to assist with creation of simple elements
pub fn new_element(name: &str, text: Option<String>) -> Element {
    Element {
        name: String::from(name),
        attributes: HashMap::new(),
        children: Vec::new(),
        text: text,
    }
}
