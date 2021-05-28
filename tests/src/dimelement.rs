use super::run_test;
use crate::svd::DimElement;

#[test]
fn decode_encode() {
    let tests = vec![(
        DimElement::builder()
            .dim(100)
            .dim_increment(4)
            .dim_index(Some(vec!["10".to_string(), "20".to_string()]))
            .build()
            .unwrap(),
        "<dimElement>
            <dim>100</dim>
            <dimIncrement>0x4</dimIncrement>
            <dimIndex>10,20</dimIndex>
        </dimElement>
        ",
    )];

    run_test::<DimElement>(&tests[..]);
}
