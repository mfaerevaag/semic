use std::io;
use std::io::Write;

use ast::CStmt;
use error::CError;
use env::SymTab;
use util;

#[derive(Clone)]
pub struct Repl {
    enabled: bool,
    verbose: bool,
    skip: usize,
    map: Vec<usize>,
    last_line: usize,
}

impl<'a> Repl {
    pub fn new(enabled: bool, program: &'a str, verbose: bool) -> Repl {
        let lines: Vec<&'a str> = program.split('\n').collect();
        let map = lines.into_iter().map(|line| line.len() + 1).collect();

        Repl {
            enabled: enabled,
            verbose: verbose,
            skip: 0,
            map: map,
            last_line: 0,
        }
    }

    pub fn show<'input>(
        &mut self,
        stmt: &'input CStmt<'input>,
        global_symtab: &'input SymTab<'input>,
        local_symtab: &'input SymTab<'input>,
    ) -> Result<(), CError>
    {
        if !self.enabled { return Ok(()); }

        let loc = match *stmt {
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

        if self.verbose {
            println!("REPL ({:?}/{:?} skip: {}) {:?}", opt, loc, self.skip, stmt);
        }

        if let Some(line) = opt  {
            if line == self.last_line {
                return Ok(());
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
                        Err(error) => return Err(CError::UnknownError(format!("Error: {}", error))),
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

                            // skip, minus the current
                            self.skip += n - 1;

                            break;
                        },
                        Some("print") => {
                            // get ident
                            let id = match arg {
                                Some(x) => x,
                                None => {
                                    println!("Incorrect command usage: try 'print <variable>");
                                    continue;
                                },
                            };

                            // get val
                            let val = match local_symtab.get_val(id) {
                                x @ Some(_) => x,
                                _ => match global_symtab.get_val(id) {
                                    x @ Some(_) => x,
                                    _ => None,
                                }
                            };

                            // print value
                            match val {
                                Some(v) => println!("{:?}", v),
                                None => println!("N\\A"),
                            };
                        },
                        Some("trace") => {
                            // get ident
                            let id = match arg {
                                Some(x) => x,
                                None => {
                                    println!("Incorrect command usage: try 'trace <variable>");
                                    continue;
                                },
                            };

                            // get val
                            let trace = match local_symtab.get_trace(id) {
                                Some(x) => x,
                                _ => match global_symtab.get_trace(id) {
                                    Some(x) => x,
                                    _ => {
                                        println!{"N\\A"};
                                        continue;
                                    }
                                }
                            };

                            for (valo, loco) in trace {
                                let locs = match loco {
                                    Some(x) => format!("{}", x),
                                    None => "N\\A".to_owned()
                                };

                                // print history
                                let vals = match valo {
                                    Some(v) => format!("{:?}", v),
                                    None => format!("N\\A"),
                                };

                                println!("{} = {} at line {}", id, vals, locs);
                            }
                        },
                        Some(x) => println!("Unknown command '{}'. Try again", x),
                        None => println!("No command given. Try again"),
                    };
                }
            }

            self.last_line = line;
        }

        Ok(())
    }
}
