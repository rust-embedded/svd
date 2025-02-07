use super::run_test;
use crate::svd::Endian;

#[test]
fn decode_encode() {
    let tests = [
        (
            Endian::Little,
            "<endian>little</endian>",
            "<endian>little</endian>",
        ),
        (Endian::Big, "<endian>big</endian>", "<endian>big</endian>"),
        (
            Endian::Selectable,
            "<endian>selectable</endian>",
            "<endian>selectable</endian>",
        ),
        (
            Endian::Other,
            "<endian>other</endian>",
            "<endian>other</endian>",
        ),
    ];

    run_test::<Endian>(&tests[..], None, None);
}
