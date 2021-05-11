use super::run_test;
use crate::svd::ModifiedWriteValues;

#[test]
fn decode_encode() {
    // FIXME: Do we need a more extensive test?
    let tests = vec![(
        ModifiedWriteValues::OneToToggle,
        "<modifiedWriteValues>oneToToggle</modifiedWriteValues>",
    )];

    run_test::<ModifiedWriteValues>(&tests[..]);
}
