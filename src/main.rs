#[macro_use]
extern crate log;
extern crate env_logger;
extern crate warlib;

use std::env;
use std::io::BufRead;

fn main() {
    pre_boot();
    let (tx, rx) = warlib::Bot::spawn();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        tx.send(line.unwrap()).unwrap()
    }

    std::thread::spawn(move || {
        for response in rx.iter() {
            println!("{}", response);
        }
    });
}

fn pre_boot() {
    env::set_var("RUST_LOG", "info");
    env_logger::init().unwrap();
    info!("pre_boot complete!");
}
