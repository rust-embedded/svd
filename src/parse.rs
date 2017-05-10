use xmltree::Element;

use errors::*;

pub fn u32(tree: &Element) -> Result<u32> {
    let text = bail_if_none!(tree.text.as_ref(), "Couldn't get `Element`"); // TODO: Fix error message

    if text.starts_with("0x") || text.starts_with("0X") {
        u32::from_str_radix(&text["0x".len()..], 16).chain_err(|| "Couldn't parse hex")
    } else if text.starts_with('#') {
        // Handle strings in the binary form of:
        // #01101x1
        // along with don't care character x (replaced with 0)
        u32::from_str_radix(&str::replace(&text["#".len()..], "x", "0"), 2)
            .chain_err(|| "Couldn't parse binary")
    } else {
        text.parse()
            .chain_err(|| format!("Failed to parse {:?}", text))
    }
}


pub fn bool(tree: &Element) -> Result<bool> {
    let text = bail_if_none!(tree.text.as_ref(), "Couldn't get bool");
    text.parse::<bool>().map_err(|e| e.into())
}

pub fn dim_index(text: &str) -> Result<Vec<String>> {
    if text.contains('-') {
        let mut parts = text.splitn(2, '-');
        let start = bail_if_none!(
            bail_if_none!(parts.next(), "Invalid dimIndex")
                .parse::<u32>()
                .ok(),
            "Couldn't parse dim start",
        );
        let end = bail_if_none!(
            bail_if_none!(parts.next(), "Invalid dimIndex")
                .parse::<u32>()
                .ok(),
            "Couldn't parse dimIndex end",
        ) + 1;

        Ok((start..end).map(|i| i.to_string()).collect())
    } else if text.contains(',') {
        Ok(text.split(',').map(|s| s.to_string()).collect())
    } else {
        unreachable!()
    }
}
