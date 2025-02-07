use super::run_test;
use crate::svd::{DimElement, ValidateLevel};

#[test]
fn decode_encode() {
    let tests = [(
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
        "<dimElement>
            <dim>2</dim>
            <dimIncrement>0x4</dimIncrement>
            <dimIndex>10,20</dimIndex>
        </dimElement>
        ",
    )];
    run_test::<DimElement>(&tests[..], None, None);

    let tests = [(
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
        "<dimElement>
            <dim>3</dim>
            <dimIncrement>0x4</dimIncrement>
            <dimIndex>3-5</dimIndex>
        </dimElement>
        ",
    )];
    run_test::<DimElement>(&tests[..], None, None);

    let tests = [(
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
        "<dimElement>
            <dim>3</dim>
            <dimIncrement>0x4</dimIncrement>
            <dimIndex>3,5,4</dimIndex>
        </dimElement>
        ",
    )];
    run_test::<DimElement>(&tests[..], None, None);

    let tests = [(
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
        "<dimElement>
            <dim>1</dim>
            <dimIncrement>0x0</dimIncrement>
            <dimIndex>3-3</dimIndex>
        </dimElement>
        ",
    )];
    run_test::<DimElement>(&tests[..], None, None);

    let parse_config = svd_parser::Config::default();
    let mut encode_config = svd_encoder::Config::default();
    encode_config.update("dim_dim", "UpperHex");
    encode_config.update("dim_increment", "LowerHex");

    let tests = [(
        DimElement::builder()
            .dim(14)
            .dim_increment(15)
            .build(ValidateLevel::Strict)
            .unwrap(),
        "<dimElement>
            <dim>14</dim>
            <dimIncrement>0xF</dimIncrement>
        </dimElement>
        ",
        "<dimElement>
            <dim>0xE</dim>
            <dimIncrement>0xf</dimIncrement>
        </dimElement>
        ",
    )];

    run_test::<DimElement>(&tests[..], Some(parse_config), Some(encode_config));
}

#[test]
fn decode_encode_one_element() {}
