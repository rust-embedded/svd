use super::run_test;
use crate::svd::ModifiedWriteValues;

#[test]
fn decode_encode() {
    // FIXME: Do we need a more extensive test?
    let tests = [(
        ModifiedWriteValues::OneToToggle,
        "<modifiedWriteValues>oneToToggle</modifiedWriteValues>",
        "<modifiedWriteValues>oneToToggle</modifiedWriteValues>",
    )];

    run_test::<ModifiedWriteValues>(&tests[..], None, None);
}
