use xmltree::Element;

pub fn u32(tree: &Element) -> Option<u32> {
    let text = try!(tree.text.as_ref());

    if text.starts_with("0x") || text.starts_with("0X") {
        u32::from_str_radix(&text["0x".len()..], 16).ok()
    } else if text.starts_with('#') {
        // Handle strings in the binary form of:
        // #01101x1
        // along with don't care character x (replaced with 0)
        u32::from_str_radix(&str::replace(&text["#".len()..], "x", "0"), 2).ok()
    } else {
        text.parse().ok()
    }
}
