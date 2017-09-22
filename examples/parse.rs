extern crate svd_parser;
extern crate xmltree;

use std::env;
use std::fs::File;
use std::io::Read;

use xmltree::Element;
use svd_parser::EnumeratedValue;
use svd_parser::Parse;
use svd_parser::error::*;

fn main() {
    let mut xml = String::new();
    File::open(env::args_os().nth(1).unwrap())
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();

    println!("{}", xml);

    let e = Element::parse(xml.as_bytes()).unwrap();

    if let Err(e) =
        EnumeratedValue::parse(&e).chain_err(|| "parsing `<enumeratedValue>`")
    {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "+ error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "| {}", e).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
