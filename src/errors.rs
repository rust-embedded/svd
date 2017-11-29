use std::fmt;
use failure::Error;

#[derive(Debug, Fail)]
pub enum TagError {
    #[fail(display = "Expected {} in `<{}>` element, found none",content, name)]
    EmptyTag {
        name: String,
        content: XmlContent, 
    },
    #[fail(display = "Expected a `<{}>` tag but found none", name)]
    MissingTag {
        name: String,
    },
}

#[derive(Debug)]
pub enum XmlContent {
    Text,
    Element,
    Unknown,
}

impl fmt::Display for XmlContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &XmlContent::Text => write!(f, "text content"),
            &XmlContent::Element => write!(f, "element contents"),
            &XmlContent::Unknown => write!(f, "contents"),
        }
    }
}

// TODO: ParseError

#[derive(Debug, Fail)]
pub enum RegisterError {
    #[fail(display = "Register #{} has no name", _0)]
    UnnamedRegister(usize, #[cause] TagError),
    #[fail(display = "In register \"{}\"", _1)]
    NamedRegister(usize,String),
}

impl RegisterError {
    pub fn from_cause(f: Error, i: usize) -> Error {
        let res = f.downcast::<TagError>();
        if let Ok(tagerror) = res {
            return RegisterError::UnnamedRegister(i,tagerror).into()
        }
        let res = res.unwrap_err().downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(RegisterError::NamedRegister(i,name)).into()
        }
        println!("\"{}\"", res.unwrap_err());
        unimplemented!()
    }
}

#[derive(Debug, Fail)]
#[fail(display = "")]
/// Internal, only to capture name
pub(crate) struct Named(pub String, pub Error);

// TODO: Put all *Error that relates to inner levels of device (and device) into one enum

#[derive(Debug, Fail)]
pub enum FieldError {
    #[fail(display = "Field #{} has no name", _0)]
    UnnamedField(usize, #[cause] TagError),
    #[fail(display = "In field \"{}\"", _1)]
    NamedField(usize,String),
}

impl FieldError {
    pub fn from_cause(f: Error, i: usize) -> Error {
        let res = f.downcast::<TagError>();
        if let Ok(tagerror) = res {
            return FieldError::UnnamedField(i,tagerror).into()
        }
        let res = res.unwrap_err().downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(FieldError::NamedField(i,name)).into()
        }
        println!("\"{}\"", res.unwrap_err());
        unimplemented!()
    }
}

#[derive(Debug,Fail)]
#[fail(display = "While parsing `<{}>` ({}) as {}", tagname, text, conv)]
pub struct ParseError {
    pub tagname: String,
    pub conv: ConvType,
    // TODO: Is this worth it?
    pub text: String,
}

#[derive(Debug)]
pub enum ConvType {
    Bool,
    U32,
    DimIndex,
}

impl fmt::Display for ConvType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ConvType::Bool => write!(f, "bool"),
            &ConvType::U32 => write!(f, "u32"),
            &ConvType::DimIndex => write!(f, "dim indices"),
        }
    }
}
