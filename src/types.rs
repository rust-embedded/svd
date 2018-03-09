// Helper traits for rust-svd

use std::collections::HashMap;
use xmltree::Element;

/// Parse trait allows SVD objects to be parsed from XML elements.
pub trait Parse {
    /// Object returned by parse method
    type Object;
    /// Parsing error
    type Error;
    /// Parse an XML/SVD element into it's corresponding Object.
    fn parse(&Element) -> Result<Self::Object, Self::Error>;
}

/// Encode trait allows SVD objects to be encoded into XML elements.
pub trait Encode {
    /// Encoding error
    type Error;
    /// Encode into an XML/SVD element
    fn encode(&self) -> Result<Element, Self::Error>;
}

/// new_element helper to create new xml elements
pub fn new_element(name: &str, text: Option<String>) -> Element {
    Element {
        name: String::from(name),
        attributes: HashMap::new(),
        children: Vec::new(),
        text: text,
    } 
}

