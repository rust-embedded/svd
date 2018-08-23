
use xmltree::Element;

/// Encode trait allows SVD objects to be encoded into XML elements.
pub trait Encode {
    /// Encoding error
    type Error;
    /// Encode into an XML/SVD element
    fn encode(&self) -> Result<Element, Self::Error>;
}

/// EncodeChildren allows SVD objects to be encoded as a list of XML elements
/// This is typically used to merge with an existing element.
pub trait EncodeChildren {
    /// Encoding error
    type Error;
    /// Encode into XML/SVD children to merge with existing object
    fn encode(&self) -> Result<Vec<Element>, Self::Error>;
}
