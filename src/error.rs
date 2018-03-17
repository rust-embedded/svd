
use xmltree::Element;
use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
#[derive(Debug)]
pub struct SVDError {
    inner: Context<SVDErrorKind>,
}

// TODO: Expand and make more complex output possible
#[derive(Clone, Debug, PartialEq, Eq, Fail)]
pub enum SVDErrorKind {
    #[fail(display = "Unknown endianness `{}`", _0)]
    UnknownEndian(String),
    // TODO: Needs context
    #[fail(display = "Missing or empty tag <{}>", _1)]
    MissingChildElement(Element, String),
    // Add TagEmpty error
    #[fail(display = "Element not Integer")]
    NonIntegerElement(Element),
    #[fail(display = "Element not boolean")]
    NonBoolElement(Element),
    #[fail(display = "NameMismatch")]
    NameMismatch(Element),
    #[fail(display = "unknown access variant found")]
    UnknownAccessType(Element),
    #[fail(display = "Bit range invalid, {:?}", _1)]
    InvalidBitRange(Element, InvalidBitRange),
    #[fail(display = "Unknown write constraint")]
    UnknownWriteConstraint(Element),
    #[fail(display = "Multiple wc found")]
    MoreThanOneWriteConstraint(Element),
    #[fail(display = "Unknown usage variant")]
    UnknownUsageVariant(Element),
    #[fail(display = "Expected a <{}>, found ...", _1)]
    NotExpectedTag(Element, String),
    // FIXME: Should not be used, only for prototyping
    #[fail(display = "{}", _0)]
    Other(String),
}

// TODO: Consider making into an Error
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidBitRange {
    Syntax,
    ParseError,
    MsbLsb,
}

impl Fail for SVDError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for SVDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
} 

impl SVDError {
    pub fn kind(&self) -> SVDErrorKind {
        self.inner.get_context().clone()
    }
}

impl From<SVDErrorKind> for SVDError {
    fn from(kind: SVDErrorKind) -> SVDError {
        SVDError { inner: Context::new(kind) }
    }
}

impl From<Context<SVDErrorKind>> for SVDError {
    fn from(inner: Context<SVDErrorKind>) -> SVDError {
        SVDError { inner }
    }
}
