
use xmltree::Element;

pub enum SVDError {
    UnknownEndian,   
    MissingChildElement(Element),
    NonIntegerElement(Element),
    NonBoolElement(Element),
    NameMismatch(Element),
}