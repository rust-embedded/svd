use super::run_test;
use crate::svd::{WriteConstraint, WriteConstraintRange};

#[test]
fn decode_encode() {
    let tests = vec![
        (
            WriteConstraint::WriteAsRead(true),
            "<writeConstraint><writeAsRead>true</writeAsRead></writeConstraint>"
        ),
        (
            WriteConstraint::UseEnumeratedValues(true),
            "<writeConstraint><useEnumeratedValues>true</useEnumeratedValues></writeConstraint>"
        ),
        (
            WriteConstraint::Range(WriteConstraintRange{min: 1, max: 10}),
            "<writeConstraint><range><minimum>1</minimum><maximum>10</maximum></range></writeConstraint>"
        ),
    ];

    run_test::<WriteConstraint>(&tests[..]);
}
