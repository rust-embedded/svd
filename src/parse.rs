use xmltree::Element;
use ElementExt;

use error::SVDError;

pub fn u32(tree: &Element) -> Option<u32> {
    let text = try!(tree.text.as_ref());

    if text.starts_with("0x") || text.starts_with("0X") {
        u32::from_str_radix(&text["0x".len()..], 16).ok()
    } else if text.starts_with('#') {
        // Handle strings in the binary form of:
        // #01101x1
        // along with don't care character x (replaced with 0)
        u32::from_str_radix(&str::replace(&text.to_lowercase()["#".len()..], "x", "0"), 2).ok()
    } else if text.starts_with("0b"){
        // Handle strings in the binary form of:
        // 0b01101x1
        // along with don't care character x (replaced with 0)
        u32::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2).ok()
    } else {
        text.parse().ok()
    }
}

pub fn bool(tree: &Element) -> Option<bool> {
    let text = try!(tree.text.as_ref());
    match text.as_ref() {
        "0" => Some(false),
        "1" => Some(true),
        _ => text.parse::<bool>().ok()
    }
}

pub fn dim_index(text: &str) -> Vec<String> {
    if text.contains('-') {
        let mut parts = text.splitn(2, '-');
        let start = try!(try!(parts.next()).parse::<u32>());
        let end = try!(try!(parts.next()).parse::<u32>()) + 1;

        (start..end).map(|i| i.to_string()).collect()
    } else if text.contains(',') {
        text.split(',').map(|s| s.to_string()).collect()
    } else {
        unreachable!()
    }
}

pub fn get_text<'a>(e: &'a Element) -> Result<String, SVDError> {
    match e.text {
        Some(s) => Ok(String::from(s)),
        None => Err(SVDError::MissingChildElement(e.clone())),
    }
}

pub fn get_child_elem<'a>(n: &str, e: &'a Element) -> Result<&'a Element, SVDError> {
    match e.get_child(n) {
        Some(s) => Ok(e),
        None => Err(SVDError::MissingChildElement(e.clone())),
    }
}

pub fn get_child_string(n: &str, e: &Element) -> Result<String, SVDError> {
    match e.get_child_text(n) {
        Some(s) => Ok(String::from(s)),
        None => Err(SVDError::MissingChildElement(e.clone())),
    }
}

pub fn get_child_u32(n: &str, e: &Element) -> Result<u32, SVDError> {
    let s = get_child_elem(n, e)?;
    match u32(&s) {
        Some(u) => Ok(u),
        None => Err(SVDError::NonIntegerElement(e.clone()))
    }
}

pub fn get_child_bool(n: &str, e: &Element) -> Result<bool, SVDError> {
    let s = get_child_elem(n, e)?;
    match bool(s) {
        Some(u) => Ok(u),
        None => Err(SVDError::NonBoolElement(e.clone()))
    }
}
