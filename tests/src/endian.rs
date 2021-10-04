use super::run_test;
use crate::svd::Endian;

#[test]
fn decode_encode() {
    let tests = vec![
        (Endian::Little, "<endian>little</endian>"),
        (Endian::Big, "<endian>big</endian>"),
        (Endian::Selectable, "<endian>selectable</endian>"),
        (Endian::Other, "<endian>other</endian>"),
    ];

    run_test::<Endian>(&tests[..]);
}
