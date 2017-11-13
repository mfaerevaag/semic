extern crate getopts;
extern crate cmm;

use getopts::Options;
use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <prog.cmm>", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // parse opts
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("i", "interactive", "run interactively");
    opts.optflag("v", "verbose", "print debug information");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    // help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    // interactive
    let interactive = matches.opt_present("i");
    // verbose
    let verbose = matches.opt_present("v");
    // program
    let path = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        process::exit(1);
    };

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
    process::exit(match cmm::run(path, prog, interactive, verbose) {
        Ok(_) => 0,
        Err(()) => 1
    });
}
