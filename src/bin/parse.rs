extern crate svd;

use std::env;
use std::fs::File;
use std::io::Read;

use svd::Device;

fn main() {
    let xml = &mut String::new();
    File::open(env::args_os().skip(1).next().unwrap()).unwrap().read_to_string(xml).unwrap();

    println!("{:#?}", Device::parse(xml));
}
