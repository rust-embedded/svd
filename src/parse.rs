use xmltree::Element;

pub fn u32(tree: &Element) -> Option<u32> {
    let text = try!(tree.text.as_ref());

    if text.starts_with("0x") || text.starts_with("0X") {
        u32::from_str_radix(&text["0x".len()..], 16).ok()
    } else {
        text.parse().ok()
    }
}
