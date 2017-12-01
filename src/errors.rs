use std::fmt;
use failure::{Error, Fail};

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

#[derive(Debug, Fail)]
pub enum PeripheralError {
    #[fail(display = "Peripheral #{} has no name", _0)]
    UnnamedPeripheral(usize, #[cause] TagError),
    #[fail(display = "In peripheral \"{}\"", _1)]
    NamedPeripheral(usize,String),
}

impl PeripheralError {
    pub fn from_cause(f: Error, i: usize) -> Error {
        let res = f.downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(PeripheralError::NamedPeripheral(i,name)).into()
        }
        let res = res.unwrap_err().downcast::<TagError>();
        if let Ok(tagerror) = res {
            return PeripheralError::UnnamedPeripheral(i,tagerror).into()
        }
        println!("\"{}\"", res.unwrap_err());
        unimplemented!()
    }
}

#[derive(Debug, Fail)]
pub enum RegisterError {
    #[fail(display = "Register #{} has no name", _0)]
    UnnamedRegister(usize, #[cause] TagError),
    #[fail(display = "In register \"{}\"", _1)]
    NamedRegister(usize,String),
}

impl RegisterError {
    pub fn from_cause(f: Error, i: usize) -> Error {
        let res = f.downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(RegisterError::NamedRegister(i,name)).into()
        }
        let res = res.unwrap_err().downcast::<TagError>();
        if let Ok(tagerror) = res {
            return RegisterError::UnnamedRegister(i,tagerror).into()
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
        let res = f.downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(FieldError::NamedField(i,name)).into()
        }
        let res = res.unwrap_err().downcast::<TagError>();
        if let Ok(tagerror) = res {
            return FieldError::UnnamedField(i,tagerror).into()
        }
        println!("\"{}\"", res.unwrap_err());
        unimplemented!()
    }
}

#[derive(Debug, Fail)]
pub enum BitRangeError {
    #[fail(display = "While parsing `<bitRange>`")]
    BitRange,
    #[fail(display = "While parsing `<msb>` and `<lsb>`")]
    MsbLsb,
    #[fail(display = "While parsing `<bitOffset>` and `<bitWidth>`")]
    BitOffsetWidth,
}

impl BitRangeError {
    pub fn from_cause(f: Error) -> Error {
        let res = f.downcast::<BitRangeParseError>();
        if let Ok(err) = res {
            return err.context(BitRangeError::BitRange).into()
            //return regname.1.context(RegisterError::NamedRegister(i,name)).into()
        }
        let res = res.unwrap_err().downcast::<MsbLsbParseError>();
        if let Ok(err) = res {
            return err.context(BitRangeError::MsbLsb).into() // FIXME: Remove duplicate
            //return regname.1.context(RegisterError::NamedRegister(i,name)).into()
        }
        //let res = f.unwrap_err().downcast::<TagError>();
        //if let Ok(tagerror) = res {
        //}
        println!("\"{}\"", res.unwrap_err());
        unimplemented!()
    }
}

#[derive(Debug,Fail)]
pub enum BitRangeParseError {
    #[fail(display = "Missing [")]
    MissingOpen,
    #[fail(display = "Missing ]")]
    MissingClose,
    #[fail(display = "An error occured while parsing")]
    ParseError(#[cause] ::std::num::ParseIntError),
    #[fail(display = "Missing something")] // FIXME: proper msg
    Other,
}

#[derive(Debug,Fail)]
#[fail(display = "When parsing msb and lsb")]
pub struct MsbLsbParseError;


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
