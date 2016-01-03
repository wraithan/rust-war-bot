#[macro_use]
extern crate log;
extern crate rand;
extern crate env_logger;

use std::env;
use std::io::BufRead;

mod warlib;
mod parser;
mod map;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init().unwrap();
    info!("Started up. System is {} bit.", std::mem::size_of::<usize>()*8);
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
