extern crate warbot;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

fn main() {
    boot();
    warbot::run();
}

fn boot() {
    env::set_var("RUST_LOG", "info");
    env_logger::init().unwrap();
    info!("Started up. System is {} bit.", std::mem::size_of::<usize>()*8);
}

#[test]
fn use_main() {
    main()
}
