use serde_yaml as yaml;
use svd_parser as svd;

use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use yaml::Value;

fn main() {
    // Collect command-line arguments.
    let argv: Vec<String> = args().collect();
    // Expect exactly one argument, with the name of the SVD file.
    // (Arg #0 is this program's name, Arg #1 is the actual argument)
    if argv.len() != 2 {
        println!("Usage: (svd2yaml) file.svd");
        return;
    }
    let svd_fn: String = argv[1].clone();

    // Open the XML-formatted SVD file and read it into a String.
    let svd_xml = &mut String::new();
    File::open(&svd_fn)
        .expect("Failed to open SVD input file")
        .read_to_string(svd_xml)
        .expect("Failed to read SVD input file to a String");

    // Use the 'svd_parser' crate to parse the file.
    let device = svd::parse(svd_xml).expect("Failed to parse the SVD file into Rust structs");

    // Convert the parsed data into YAML format.
    let v: Value = yaml::to_value(device).expect("Failed to parse Rust structs into YAML format");

    // Write the YAML-formatted device description to a file.
    let yaml_fn: String = svd_fn + ".yaml";
    File::create(yaml_fn)
        .expect("Failed to open YAML output file")
        .write_all(yaml::to_string(&v).unwrap().as_bytes())
        .expect("Failed to write to YAML output file");
}
