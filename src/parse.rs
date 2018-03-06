use xmltree::Element;
use failure::{Error, err_msg, ResultExt};
use errors;
use ElementExt;
pub fn u32(tree: &Element) -> Result<u32, Error> { // FIXME: Fix messages
    let name = tree.name.clone();
    let text = tree.get_self_text()?;
    Ok(_u32(text).with_context(|_| errors::ParseError {
        tagname: name,
        conv: errors::ConvType::U32,
        text: text.clone(),
    })?)
}
fn _u32(text: &str) -> Result<u32, Error> {
    if text.starts_with("0x") || text.starts_with("0X") {
        Ok(u32::from_str_radix(&text["0x".len()..], 16)?)
    } else if text.starts_with('#') {
        // Handle strings in the binary form of:
        // #01101x1
        // along with don't care character x (replaced with 0)
        Ok(u32::from_str_radix(&str::replace(&text.to_lowercase()["#".len()..], "x", "0"), 2)?)
    } else if text.starts_with("0b"){
        // Handle strings in the binary form of:
        // 0b01101x1
        // along with don't care character x (replaced with 0)
        Ok(u32::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2)?)
    } else {
        Ok(text.parse()?)
    }
}

pub fn u32_strict(tree: &Element) -> Result<u32, Error> { // FIXME: Fix messages
    let name = tree.name.clone();
    let text = tree.get_self_text()?;
    Ok(text.parse::<u32>().with_context(|_| errors::ParseError {
        tagname: name,
        conv: errors::ConvType::U32,
        text: text.clone(),
    })?)
}

pub fn bool(tree: &Element) -> Result<bool, Error> { // FIXME: Fix messages
    let name = tree.name.clone();
    let text = tree.get_self_text()?;
    Ok(_bool(text).with_context(|_| errors::ParseError {
        tagname: name,
        conv: errors::ConvType::Bool,
        text: text.clone(),
    })?)
}
fn _bool(text: &str) -> Result<bool,Error> {
    // FIXME: parse::bool should take a &str
    match text {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Ok(text.parse::<bool>()?)
    }
}

pub fn dim_index(tree: &Element) -> Result<Vec<String>,Error> {
    let name = tree.name.clone();
    let text = tree.get_self_text()?;
    Ok(_dim_index(text).with_context(|_| errors::ParseError {
        tagname: name,
        conv: errors::ConvType::DimIndex,
        text: text.clone(),
    })?)
}
fn _dim_index(text: &str) -> Result<Vec<String>,Error> {
    if text.contains('-') {
        let mut parts = text.splitn(2, '-');
        let start = parts.next().ok_or(err_msg("couldn't advance"))?.parse::<u32>()?;
        let end = parts.next().ok_or(err_msg("couldn't advance"))?.parse::<u32>()? + 1;

        Ok((start..end).map(|i| i.to_string()).collect())
    } else if text.contains(',') {
        Ok(text.split(',').map(|s| s.to_string()).collect())
    } else {
        unreachable!()
    }
}
