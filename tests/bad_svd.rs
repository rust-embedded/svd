extern crate svd_parser as svd;
extern crate failure;

use failure::Fail;

#[test]
#[should_panic]
fn peripheral_name_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/peripheral-name-missing.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

#[test]
#[should_panic]
fn peripheral_name_empty() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/peripheral-name-empty.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

#[test]
#[should_panic]
fn peripherals_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/peripherals-missing.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

#[test]
#[should_panic]
fn register_name_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/register-name-missing.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

#[test]
#[should_panic]
fn field_name_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/field-name-missing.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

#[test]
#[should_panic]
fn bitoffset_invalid() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/bitoffset-invalid.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

#[test]
#[should_panic]
fn enumerated_value_name_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/enumerated-value-name-missing.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

#[test]
#[should_panic]
fn bad_register_size() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/bad-register-size.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

#[test]
#[should_panic]
fn arm_sample_faulty() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/ARM_Sample_faulty.svd"));
    if let Err(e) = svd::parse(xml) {
        print_causes(e.cause());
        panic!()
    }
}

/// Used for debugging errors
fn print_causes(mut fail: &Fail) {
    println!("{}", &fail);
    while let Some(cause) = fail.cause() {
        fail = cause;
        println!("{}", &fail);
    }
}
