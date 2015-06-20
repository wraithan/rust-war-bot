#[macro_use]
extern crate log;
extern crate env_logger;

extern crate warlib;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Once, ONCE_INIT};
use std::sync::mpsc::{Receiver, TryRecvError, channel};

include!(concat!(env!("OUT_DIR"), "/tests.rs"));

static START: Once = ONCE_INIT;

fn run_file(name: &str) {
    START.call_once(|| {
        env::set_var("RUST_LOG", "warn");
        env_logger::init().unwrap();
    });

    let mut file_path = Path::new("tests/fodder").join(name);
    file_path.set_extension("txt");

    let mut file = match File::open(&file_path) {
        Err(e) => {
            println!("Couldn't open {}: {}", file_path.display(), e);
            panic!(e);
        },
        Ok(f) => f
    };
    let (tx, rx) = warlib::Bot::spawn();

    let mut last = String::new();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for raw_line in contents.split('\n') {
        let line = raw_line.trim().to_owned();
        if line.len() == 0 || line.starts_with("Output") || line.starts_with("Round"){
            continue;
        } else if line.starts_with("#") {
            if line.starts_with("# Valid") {
                run_spec(line, &last)
            }
        } else if gets_response(&line) {
            tx.send(line).unwrap();
            let timer = set_timer(1000);
            loop {
                match rx.try_recv() {
                    Ok(response) => {
                        last = response;
                        if let Ok(extra_response) = rx.try_recv() {
                            panic!("got extra line: {}", extra_response);
                        }
                        break;
                    },
                    Err(e) => if let TryRecvError::Disconnected = e {
                        panic!("bot crashed")
                    }
                }
                if let Ok(_) = timer.try_recv() {
                    panic!("timeout exceeded")
                }

                std::thread::yield_now()
            }
        } else {
            tx.send(line).unwrap();
        }
    }
}

enum TestSpec {
    Contains(String),
    ExactlyMatches(String),
    NotContains(String)
}

fn run_spec (line: String, last: &String) {
    let spec = line.trim_left_matches("# Valid: ").to_owned();
    match parse_spec(spec) {
        TestSpec::Contains(ref expected) => {
            if !last.contains(expected) {
                panic!("Expected {} to contain {}", last, expected);
            }
        },
        TestSpec::ExactlyMatches(ref expected) => assert_eq!(last, expected),
        TestSpec::NotContains(ref expected) => {
            if last.contains(expected) {
                panic!("Expected {} to not contain {}", last, expected);
            }
        }
    };
}

fn parse_spec (mut spec: String) -> TestSpec {
    match spec.chars().nth(0).unwrap() {
        '!' => {
            spec.remove(0);
            match parse_spec(spec) {
                TestSpec::Contains(expected) => TestSpec::NotContains(expected),
                _ => panic!("Not Implemented")
            }
        },
        '[' => {
            let container: &[_] = &['[', ']'];
            TestSpec::Contains(spec.trim_matches(container).to_owned())
        },
        _ => TestSpec::ExactlyMatches(spec)
    }
}

fn gets_response(line: &String) -> bool {
    if line.starts_with("go") || line.starts_with("pick_starting_region") {
        return true;
    }
    false
}

fn set_timer(duration: u32) -> Receiver<()> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        std::thread::sleep_ms(duration);
        tx.send(()).unwrap_or(());
    });
    rx
}
