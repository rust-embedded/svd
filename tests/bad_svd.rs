extern crate svd_parser as svd;
extern crate failure;

use svd::errors as err;
use failure::Fail;

#[test]
fn peripheral_name_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/peripheral-name-missing.svd"));
    //if let Err(err::PeripheralError::UnnamedPeripheral(_, err::TagError::MissingTag)) = svd::parse(xml) {
    let res = svd::parse(xml);
    if let &err::PeripheralError::UnnamedPeripheral(i,ref e) = res.unwrap_err().downcast_ref::<err::PeripheralError>().unwrap() {
        assert_eq!(i, 1);
        assert_eq!(e, &err::TagError::MissingTag{ name: "name".into()});
    }
}

#[test]
fn peripheral_name_empty() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/peripheral-name-empty.svd"));
    let res = svd::parse(xml);
    if let &err::PeripheralError::UnnamedPeripheral(i,ref e) = res.unwrap_err().downcast_ref::<err::PeripheralError>().unwrap() {
        assert_eq!(i, 1);
        assert_eq!(e, &err::TagError::EmptyTag{ name: "name".into(), content: err::XmlContent::Text});
    } else {
        panic!()
    }
}

#[test]
fn peripherals_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/peripherals-missing.svd"));
    let res = svd::parse(xml);
    if let &err::TagError::EmptyTag{ref name, ref content} = res.unwrap_err().downcast_ref::<err::TagError>().unwrap() {
        assert_eq!(name, "peripherals");
        assert_eq!(content, &err::XmlContent::Element);
    } else {
        panic!()
    }
}

#[test]
#[ignore]
fn register_name_missing() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/register-name-missing.svd"));
    let res = svd::parse(xml).unwrap_err();
    // FIXME: Printing
    println!("{:?}", res.causes().collect::<Vec<&failure::Fail>>());
    // gives me 
    // > In peripheral "GPIOA", UnnamedRegister(1, MissingTag { name: "name" }), MissingTag { name: "name" }]
    // however, to downcast, we have to actually go through a Context<PeripheralError>
    let res = res.downcast_ref::<failure::Context<err::PeripheralError>>().unwrap().cause().expect("2").downcast_ref::<err::PeripheralError>().expect("Ehm");
    if let &err::PeripheralError::NamedPeripheral(ref p_name) = res {
        assert_eq!(p_name, "GPIOA");
        println!("{:?}", res);
        if let &err::RegisterClusterError::UnnamedRegister(i, ref _tagerr) = res.cause().expect("3").downcast_ref::<err::RegisterClusterError>().expect("4") {
            assert_eq!(i,1);
            // assert that tagerr is correct
        } else {
            panic!()
        }
    } else {
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
#[ignore]
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

#[test]
#[should_panic]
fn nrf51_faulty() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bad_svd/nrf51_faulty.svd"));
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
