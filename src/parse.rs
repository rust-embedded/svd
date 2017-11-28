use xmltree::Element;
use failure::{Error, err_msg};
pub fn u32(tree: &Element) -> Result<u32, Error> { // FIXME: Fix messages
    let text = tree.text.as_ref().ok_or(format_err!("couldn't get {:?}",tree.name))?; // FIXME

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

pub fn bool(tree: &Element) -> Result<bool,Error> {
    let text = tree.text.as_ref().ok_or(format_err!("couldn't get {:?}",tree.name))?; // FIXME
    match text.as_ref() {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Ok(text.parse::<bool>()?)
    }
}

pub fn dim_index(text: &str) -> Result<Vec<String>,Error> {
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
