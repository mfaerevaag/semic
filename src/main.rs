extern crate cmm;

use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    // parse arg
    let filename = match env::args().nth(1) {
        Some(s) => s,
        None => {
            println!("Usage: {} <filename>", env::args().nth(0).unwrap());
            process::exit(1);
        }
    };

    // try to get file
    let mut file = match File::open(filename.clone()) {
        Ok(f) => f,
        Err(err) => {
            println!("Error: failed opening file '{}' ({})", filename, err.to_string());
            process::exit(1);
        }
    };

    // read file
    let mut prog = String::new();
    file.read_to_string(&mut prog).unwrap();

    // run
    process::exit(match cmm::run(filename, prog) {
        Ok(_) => 0,
        Err(()) => 1
    });
}
