use std::io;
use std::io::Write;

use ast::CStmt;
use util;

#[derive(Clone)]
pub struct Repl {
    enabled: bool,
    skip: usize,
    map: Vec<usize>,
    last_line: usize,
}

impl<'a> Repl {
    pub fn new(enabled: bool, program: &'a str) -> Repl {
        let lines: Vec<&'a str> = program.split('\n').collect();
        let map = lines.into_iter().map(|line| line.len() + 1).collect();

        Repl {
            enabled: enabled,
            skip: 0,
            map: map,
            last_line: 0,
        }
    }

    pub fn show<'input>(&mut self, stmt: CStmt<'input>) -> Result<(), CError> {
        let loc = match stmt {
            CStmt::Decl((l, _), ..) |
            CStmt::Assign((l, _), ..) |
            CStmt::While((l, _), ..) |
            CStmt::Call((l, _), ..) |
            CStmt::Return((l, _), ..) |
            CStmt::If((l, _), ..) |
            CStmt::Print((l, _), ..) => Some(l),
            _ => None,
        };
        let opt = match loc {
            Some(x) => util::line_from(x, &self.map),
            None => None
        };

        println!("REPL ({:?}/{:?} skip: {}) {:?}", opt, loc, self.skip, stmt);

        if let Some(line) = opt  {
            if line == self.last_line {
                return;
            }

            // blank lines counts when skipping
            if line > self.last_line {
                for _ in 0..(line - self.last_line) {
                    if self.skip > 0 {
                        self.skip -= 1;
                    } else {
                        break;
                    }
                }
            }

            // check if should skip
            if self.skip > 0 {
                println!("-> skip {}", self.skip);
                self.skip -= 1;
            } else {
                loop {
                    print!(">> ");
                    let _ = io::stdout().flush().unwrap(); // TODO: error handling

                    // read input
                    let mut input = String::new();
                    let (command, arg) = match io::stdin().read_line(&mut input) {
                        Err(error) => panic!("Error: {}", error),
                        Ok(_) => {
                            let mut matches = input.split_whitespace();
                            (matches.next(), matches.next())
                        }
                    };

                    // match command
                    match command {
                        Some("next") => {
                            // get number
                            let n: usize = match arg {
                                Some(x) => match x.parse() {
                                    Ok(n) => n,
                                    Err(_) => {
                                        println!("Incorrect command usage: try 'next [lines]");
                                        continue;
                                    }
                                },
                                None => 1,
                            };

                            // do nothing of zero
                            if n == 0 { continue; }

                            println!("TODO: next {}", n);
                            break;
                        },
                        Some("print") => {
                            println!("TODO: print");
                            break;
                        },
                        Some("trace") => {
                            println!("TODO: trace");
                            break;
                        },
                        Some(x) => println!("Unknown command '{}'. Try again", x),
                        None => println!("No command given. Try again"),

                    };
                }
            }

            self.last_line = line;
        }
    }
}
