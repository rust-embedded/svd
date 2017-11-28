extern crate svd_parser as svd;
extern crate failure;

use failure::Fail;
#[test]
#[should_panic]
fn peripheral_name_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/peripheral-name-missing.svd"));
    match svd::parse(xml) {
        Err(e) => {
            print_causes(e.cause());
            panic!()
        },
        _ => (),
    }
}

#[test]
#[should_panic]
fn peripheral_name_empty() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/peripheral-name-empty.svd"));
    match svd::parse(xml) {
        Err(e) =>{
            print_causes(e.cause());
            panic!()
        },
        _ => (),
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
