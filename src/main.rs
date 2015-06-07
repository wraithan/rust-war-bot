extern crate warbot;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::io::BufRead;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init().unwrap();
    info!("Started up. System is {} bit.", std::mem::size_of::<usize>()*8);
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut bot = warbot::Bot::new();
        loop {
            match rx.try_recv() {
                Ok(line) => bot.read_line(line),
                Err(_) => bot.calculate()
            }
        }
    });
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        tx.send(line.unwrap()).unwrap()
    }
}
