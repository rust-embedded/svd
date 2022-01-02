use serde_yaml as yaml;
use svd_parser as svd;

use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use yaml::Value;

fn main() {
    // Collect command-line arguments.
    let mut args = args();
    // Expect exactly one argument, with the name of the SVD file.
    // (Arg #0 is this program's name, Arg #1 is the actual argument)
    let svd_fn = if let (Some(_), Some(arg1), None) = (args.next(), args.next(), args.next()) {
        PathBuf::from(arg1)
    } else {
        println!("Usage: (svd2yaml) file.svd");
        return;
    };

    // Open the XML-formatted SVD file and read it into a String.
    let mut svd_xml = String::new();
    File::open(&svd_fn)
        .expect("Failed to open SVD input file")
        .read_to_string(&mut svd_xml)
        .expect("Failed to read SVD input file to a String");

    // Use the 'svd_parser' crate to parse the file.
    let device = svd::parse(&mut svd_xml).expect("Failed to parse the SVD file into Rust structs");

    // Convert the parsed data into YAML format.
    let v: Value =
        yaml::to_value(device).expect("Failed to serialize Rust structs into YAML format");

    // Write the YAML-formatted device description to a file.
    let mut yaml_fn = svd_fn.clone();
    yaml_fn.set_extension("yaml");
    File::create(&yaml_fn)
        .expect("Failed to open YAML output file")
        .write_all(yaml::to_string(&v).unwrap().as_bytes())
        .expect("Failed to write to YAML output file");
}
