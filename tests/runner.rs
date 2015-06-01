use std::fs::File;
use std::io::Read;
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/tests.rs"));

fn run_file (name: &str) {
    let mut file_path = Path::new("tests/fodder").join(name);
    file_path.set_extension("txt");

    let mut file = match File::open(&file_path) {
        Err(e) => {
            println!("Couldn't open {}: {}", file_path.display(), e);
            panic!(e);
        },
        Ok(f) => f
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
