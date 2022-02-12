use super::run_test;
use crate::svd::{DimElement, ValidateLevel};

#[test]
fn decode_encode() {
    let tests = vec![(
        DimElement::builder()
            .dim(2)
            .dim_increment(4)
            .dim_index(Some(vec!["10".to_string(), "20".to_string()]))
            .build(ValidateLevel::Strict)
            .unwrap(),
        "<dimElement>
            <dim>2</dim>
            <dimIncrement>0x4</dimIncrement>
            <dimIndex>10,20</dimIndex>
        </dimElement>
        ",
    )];
    run_test::<DimElement>(&tests[..]);

    let tests = vec![(
        DimElement::builder()
            .dim(3)
            .dim_increment(4)
            .dim_index(Some(vec![
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
            ]))
            .build(ValidateLevel::Strict)
            .unwrap(),
        "<dimElement>
            <dim>3</dim>
            <dimIncrement>0x4</dimIncrement>
            <dimIndex>3-5</dimIndex>
        </dimElement>
        ",
    )];
    run_test::<DimElement>(&tests[..]);

    let tests = vec![(
        DimElement::builder()
            .dim(3)
            .dim_increment(4)
            .dim_index(Some(vec![
                "3".to_string(),
                "5".to_string(),
                "4".to_string(),
            ]))
            .build(ValidateLevel::Strict)
            .unwrap(),
        "<dimElement>
            <dim>3</dim>
            <dimIncrement>0x4</dimIncrement>
            <dimIndex>3,5,4</dimIndex>
        </dimElement>
        ",
    )];
    run_test::<DimElement>(&tests[..]);

    let tests = vec![(
        DimElement::builder()
            .dim(1)
            .dim_increment(0)
            .dim_index(Some(vec!["3".to_string()]))
            .build(ValidateLevel::Strict)
            .unwrap(),
        "<dimElement>
            <dim>1</dim>
            <dimIncrement>0x0</dimIncrement>
            <dimIndex>3-3</dimIndex>
        </dimElement>
        ",
    )];
    run_test::<DimElement>(&tests[..]);
}

#[test]
fn decode_encode_one_element() {}
