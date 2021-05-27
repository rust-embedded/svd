use serde_json as json;
use svd_parser as svd;

use json::Value;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // Collect command-line arguments.
    let argv: Vec<String> = args().collect();
    // Expect exactly one argument, with the name of the SVD file.
    // (Arg #0 is this program's name, Arg #1 is the actual argument)
    if argv.len() != 2 {
        println!("Usage: (svd2json) file.svd");
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

    // Convert the parsed data into JSON format.
    let v: Value = json::to_value(device).expect("Failed to parse Rust structs into JSON format");

    // Write the JSON-formatted device description to a file.
    let json_fn: String = svd_fn + ".json";
    File::create(json_fn)
        .expect("Failed to open JSON output file")
        .write_all(json::to_string_pretty(&v).unwrap().as_bytes())
        .expect("Failed to write to JSON output file");
}
