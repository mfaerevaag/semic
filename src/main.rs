extern crate getopts;
extern crate semic;

use getopts::Options;
use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <prog.semic>", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // parse opts
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("d", "debug", "interactive debug");
    opts.optflag("v", "verbose", "print debug information");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(1);
        }
    };
    // help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    // debug
    let debug = matches.opt_present("d");
    // verbose
    let verbose = matches.opt_present("v");
    // program
    let path = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        process::exit(1);
    };
    // args
    let argv = matches.free[1..].to_vec();

    // try to get file
    let mut file = match File::open(path.clone()) {
        Ok(f) => f,
        Err(err) => {
            println!("Error: failed opening file '{}' ({})", path, err.to_string());
            process::exit(1);
        }
    };

    // read file
    let mut prog = String::new();
    file.read_to_string(&mut prog).unwrap();

    // run
    process::exit(match semic::run(path, prog, argv, debug, verbose) {
        Ok(_) => 0,
        Err(()) => 1
    });
}
