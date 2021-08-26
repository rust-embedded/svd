use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .unwrap_or_else(|| panic!("Please, specify yaml file path as first argument"));
    svdpatch::process_file(Path::new(path))
}
