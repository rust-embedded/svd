
use xmltree::Element;


#[derive(Clone, Debug, PartialEq)]
pub enum SVDError {
    UnknownEndian,   
    MissingChildElement(Element),
    NonIntegerElement(Element),
    NonBoolElement(Element),
    NameMismatch(Element),
}