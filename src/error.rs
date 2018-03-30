
use xmltree::{Element, ParseError};
use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};


#[derive(Debug)]
pub struct SVDError {
    inner: Context<SVDErrorKind>,
}

// TODO: Expand and make more complex output possible.
// We can use the `Element` to output name (if available) and etc.
#[derive(Clone, Debug, PartialEq, Eq, Fail)]
pub enum SVDErrorKind {
    #[fail(display = "Unknown endianness `{}`", _0)]
    UnknownEndian(String),
    // TODO: Needs context
    // TODO: Better name
    #[fail(display = "Expected a <{}> tag, found none", _1)]
    MissingTag(Element, String),
    #[fail(display = "Expected content in <{}> tag, found none", _1)]
    EmptyTag(Element, String),
    #[fail(display = "ParseError")]
    ParseError(Element),
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
    #[fail(display = "Invalid RegisterCluster (expected register or cluster), found {}", _1)]
    InvalidRegisterCluster(Element, String),
    #[fail(display = "Invalid modifiedWriteValues variant, found {}", _1)]
    InvalidModifiedWriteValues(Element, String),
    #[fail(display = "encoding method not implemented for svd object {}", _0)]
    EncodeNotImplemented(String),
    #[fail(display = "Error parsing SVD XML")]
    FileParseError,
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

impl From<ParseError> for SVDError {
    fn from(e: ParseError) -> SVDError {
        SVDError { inner: e.context(SVDErrorKind::FileParseError) }
    }
}
