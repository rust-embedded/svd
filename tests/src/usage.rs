use super::run_test;
use crate::svd::Usage;

#[test]
fn decode_encode() {
    let tests = vec![
        (Usage::Read, "<usage>read</usage>", "<usage>read</usage>"),
        (Usage::Write, "<usage>write</usage>", "<usage>write</usage>"),
        (
            Usage::ReadWrite,
            "<usage>read-write</usage>",
            "<usage>read-write</usage>",
        ),
    ];

    run_test::<Usage>(&tests[..], None, None);
}
