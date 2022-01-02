use serde_json as json;
use svd_parser as svd;

use json::Value;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    // Collect command-line arguments.
    let mut args = args();
    // Expect exactly one argument, with the name of the SVD file.
    // (Arg #0 is this program's name, Arg #1 is the actual argument)
    let svd_fn = if let (Some(_), Some(arg1), None) = (args.next(), args.next(), args.next()) {
        PathBuf::from(arg1)
    } else {
        println!("Usage: (svd2json) file.svd");
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

    // Convert the parsed data into JSON format.
    let v: Value =
        json::to_value(device).expect("Failed to serialize Rust structs into JSON format");

    // Write the JSON-formatted device description to a file.
    let mut json_fn = svd_fn.to_path_buf();
    json_fn.set_extension("json");
    File::create(json_fn)
        .expect("Failed to open JSON output file")
        .write_all(json::to_string_pretty(&v).unwrap().as_bytes())
        .expect("Failed to write to JSON output file");
}
