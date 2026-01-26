#![allow(dead_code)]
#![allow(unused)]

use steganors::Result;

mod args;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Err(e) = args::run(&args) {
        eprintln!("Erreur: {e}");
        std::process::exit(1);
    }
}
