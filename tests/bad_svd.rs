extern crate failure;
extern crate svd_parser as svd;

use failure::Fail;

#[test]
fn arm_sample_faulty() {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/ARM_Sample_faulty.svd"
    ));
    if let Err(e) = svd::parse(xml) {
        for e in Fail::iter_chain(&e) {
            println!("{}", e);
        }
    } else {
        panic!()
    }
}
