use super::run_test;
use crate::svd::{DimElement, Register, RegisterInfo, ValidateLevel};

#[test]
fn decode_encode() {
    let tests = vec![(
        Register::Array(
            RegisterInfo::builder()
                .name("MODE%s".to_string())
                .address_offset(8)
                .build(ValidateLevel::Strict)
                .unwrap(),
            DimElement::builder()
                .dim(2)
                .dim_increment(4)
                .dim_index(Some(vec!["10".to_string(), "20".to_string()]))
                .build(ValidateLevel::Strict)
                .unwrap(),
        ),
        "
        <register>
          <dim>2</dim>
          <dimIncrement>0x4</dimIncrement>
          <dimIndex>10,20</dimIndex>
          <name>MODE%s</name>
          <addressOffset>0x8</addressOffset>
        </register>
        ",
    )];
    run_test::<Register>(&tests[..]);
}
