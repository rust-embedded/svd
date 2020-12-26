use svd_parser as svd;

#[test]
fn arm_sample_faulty() {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/ARM_Sample_faulty.svd"
    ));
    if let Err(e) = svd::parse(xml) {
        for e in e.chain() {
            println!("{}", e);
        }
    } else {
        panic!()
    }
}