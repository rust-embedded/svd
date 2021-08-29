use crate::EncodeChildren;
use crate::parse::Parse;
use crate::svd::{Access, RegisterProperties};
use xmltree::Element;

#[test]
fn decode_encode() {
    let example = String::from(
        "
        <mock>
            <size>64</size>
            <access>read-only</access>
            <resetValue>0x11223344</resetValue>
            <resetMask>0xFFFFFFFF</resetMask>
        </mock>
    ",
    );

    let mut expected = RegisterProperties::default();
    expected.size = Some(64);
    expected.reset_value = Some(0x11223344);
    expected.reset_mask = Some(0xffffffff);
    expected.access = Some(Access::ReadOnly);

    let tree1 = Element::parse(example.as_bytes()).unwrap();

    let parsed = RegisterProperties::parse(&tree1).unwrap();
    assert_eq!(parsed, expected, "Parsing tree failed");

    let mut tree2 = Element::new("mock");
    tree2.children = parsed.encode().unwrap();
    assert_eq!(tree1, tree2, "Encoding value failed");
}
