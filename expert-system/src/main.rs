extern crate expert_system;

use std::env;
use std::process;

fn main() {
    let filename = expert_system::get_filename(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = expert_system::run(filename) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
