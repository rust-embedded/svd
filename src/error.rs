
use xmltree::Element;


#[derive(Clone, Debug, PartialEq)]
pub enum SVDError {
    UnknownEndian,   
    MissingChildElement(Element),
    NonIntegerElement(Element),
    NonBoolElement(Element),
    NameMismatch(Element),
    UnknownAccessType(Element),
    InvalidBitRange(Element),
    UnknownWriteConstraint(Element),
    MoreThanOneWriteConstraint(Element),
    UnknownUsageVariant(Element),
    NotEnumeratedValue(Element),
}