use super::run_test;
use crate::svd::Access;

#[test]
fn decode_encode() {
    let tests = vec![
        (
            Access::ReadOnly,
            "<access>read-only</access>",
            "<access>read-only</access>",
        ),
        (
            Access::ReadWrite,
            "<access>read-write</access>",
            "<access>read-write</access>",
        ),
        (
            Access::ReadWriteOnce,
            "<access>read-writeOnce</access>",
            "<access>read-writeOnce</access>",
        ),
        (
            Access::WriteOnly,
            "<access>write-only</access>",
            "<access>write-only</access>",
        ),
        (
            Access::WriteOnce,
            "<access>writeOnce</access>",
            "<access>writeOnce</access>",
        ),
    ];

    run_test::<Access>(&tests[..], None, None);
}
