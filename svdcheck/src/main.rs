use svd_parser as svd;

use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("svdcheck")
        .about("Parse SVD file and check for errors")
        .arg(
            Arg::with_name("input")
                .help("Input SVD file")
                .short("i")
                .takes_value(true)
                .value_name("FILE"),
        )
        .get_matches();

    let svd_fn = matches
        .value_of("input")
        .expect("Usage: svdcheck -i file.svd");

    println!("Processing {}", svd_fn);

    // Open the XML-formatted SVD file and read it into a String.
    let svd_xml = &mut String::new();
    File::open(&svd_fn)
        .expect("Failed to open SVD input file")
        .read_to_string(svd_xml)
        .expect("Failed to read SVD input file to a String");

    // Use the 'svd_parser' crate to parse the file.
    let mut config = svd::Config::default();
    config.validate_level = svd::ValidateLevel::Strict;
    let _device = svd::parse_with_config(svd_xml, &config)
        .expect("Failed to parse the SVD file into Rust structs");
        
    println!("OK!");
}
