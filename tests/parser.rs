extern crate warlib;

use std::fs::File;
use std::io::Read;
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/tests.rs"));

fn run_file(name: &str) {
    let mut file_path = Path::new("tests/fodder").join(name);
    file_path.set_extension("txt");

    let mut file = match File::open(&file_path) {
        Err(e) => {
            println!("Couldn't open {}: {}", file_path.display(), e);
            panic!(e);
        },
        Ok(f) => f
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for raw_line in contents.split('\n') {
        let line = raw_line.trim().to_owned();
        if line.len() == 0 {
            continue;
        } else if line.starts_with("Output") {
            continue;
        } else if line.starts_with("Round") {
            continue;
        } else if line.starts_with("#") {
            continue;
        } else if let Err(e) = warlib::parser::parse(line) {
            panic!("{:?}", e)
        }
    }
}
