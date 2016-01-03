extern crate glob;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use glob::glob;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tests.rs");
    let mut f = File::create(&dest_path).unwrap();

    for path in glob("tests/fodder/*.txt").unwrap() {
        writeln!(
            &mut f,
            "#[test]\nfn {0}() {{run_file(\"{0}\");}}",
            path.unwrap().file_stem().unwrap().to_str().unwrap()
        ).unwrap();
    }
}
