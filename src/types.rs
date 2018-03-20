// Helper traits for rust-svd

use std::fmt::Debug;
use std::collections::HashMap;
use xmltree::Element;

/// Parse trait allows SVD objects to be parsed from XML elements.
pub trait Parse {
    /// Object returned by parse method
    type Object;
    /// Parsing error
    type Error;
    /// Parse an XML/SVD element into it's corresponding `Object`.
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

use ::error::SVDError;

// Generic test function
pub fn test<T: Parse<Error=SVDError, Object=T> + Encode<Error=SVDError> + Debug + PartialEq>(tests: &[(T, &str)]) {
    for t in tests {
        let tree1 = Element::parse(t.1.as_bytes()).unwrap();
        let elem = T::parse(&tree1).unwrap();
        assert_eq!(elem, t.0, "Error parsing xml` (mismatch between parsed and expected)");
        let tree2 = elem.encode().unwrap();
        assert_eq!(tree1, tree2, "Error encoding xml (mismatch between encoded and original)");
    };
}
