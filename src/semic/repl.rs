use std::process;
use std::io;
use std::io::Write;

use ast::CStmt;
use error::CError;
use env::SymTab;
use util;

#[derive(Clone)]
pub struct Repl {
    verbose: bool,
    skip: usize,
    map: Vec<usize>,
    last_line: usize,

}

impl<'a> Repl {
    pub fn new(program: &'a str, verbose: bool) -> Repl {
        let lines: Vec<&'a str> = program.split('\n').collect();
        let map = lines.into_iter().map(|line| line.len() + 1).collect();

        Repl {
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
        let lineo = match loc {
            Some(x) => util::line_from(x, &self.map),
            None => None
        };

        if self.verbose {
            println!(" REPL ({:?}/{:?} skip: {}) {:?}", lineo, loc, self.skip, stmt);
        }

        if let Some(line) = lineo  {
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
                self.skip -= 1;
            } else {
                self.read(global_symtab, local_symtab, false)?;
            }

            self.last_line = line;
        }

        Ok(())
    }

    pub fn finished<'input>(
        &mut self,
        global_symtab: &'input SymTab<'input>,
        local_symtab: &'input SymTab<'input>,
    ) -> Result<(), CError>
    {
        println!(" End of program");

        self.read(global_symtab, local_symtab, true)?;

        Ok(())
    }

    fn read<'input>(
        &mut self,
        global_symtab: &'input SymTab<'input>,
        local_symtab: &'input SymTab<'input>,
        finished: bool
    ) -> Result<(), CError> {
        loop {
            print!(">> ");
            let _ = io::stdout().flush().unwrap(); // TODO: error handling

            // read input
            let mut input = String::new();
            let (command, arg) = match io::stdin().read_line(&mut input) {
                Err(error) => return Err(CError::UnknownError(error.to_string())),
                Ok(_) => {
                    let mut matches = input.split_whitespace();
                    (matches.next(), matches.next())
                }
            };

            // match command
            match command {
                Some("next") | Some("n") => {
                    // get number
                    let n: usize = match arg {
                        Some(x) => match x.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                println!(" Incorrect command usage: try 'next [lines]'");
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
                Some("print") | Some("p") => {
                    // get ident
                    let id = match arg {
                        Some(x) => x,
                        None => {
                            println!(" Invalid typing of the variable name");
                            continue;
                        },
                    };

                    match local_symtab.get_type(id) {
                        Some(_) => match local_symtab.get_val(id) {
                            Some(x) => println!(" {:?}", x),
                            None => println!(" N\\A"),
                        },
                        None => match global_symtab.get_type(id) {
                            Some(_) => match global_symtab.get_type(id) {
                                Some(x) => println!(" {:?} (global)", x),
                                None => println!(" N\\A (global)"),
                            },
                            None => match local_symtab.get_val_parent(id) {
                                Some(x) => println!(" {:?} (invisible)", x),
                                None => println!(" Not declared"),
                            }
                        }
                    };
                },
                Some("trace") | Some("t") => {
                    // get ident
                    let id = match arg {
                        Some(x) => x,
                        None => {
                            println!(" Invalid typing of the variable name");
                            continue;
                        },
                    };

                    // get val
                    let trace = match local_symtab.get_trace(id) {
                        Some(x) => x,
                        _ => match global_symtab.get_trace(id) {
                            Some(x) => x,
                            _ => {
                                println!{" N\\A"};
                                continue;
                            }
                        }
                    };

                    for (valo, loco) in trace {
                        let val = match valo {
                            Some(v) => format!("{:?}", v),
                            None => format!("N\\A"),
                        };

                        let lineo = match loco {
                            Some(x) => util::line_from(x, &self.map),
                            None => None
                        };

                        match lineo {
                            Some(line) => println!(" {} = {} at line {}", id, val, line),
                            None => println!(" {} = {}", id, val)
                        }
                    }
                },
                Some("quit") | Some("q") => {
                    println!(" Bye, bye");
                    if finished {
                        break;
                    } else {
                        process::exit(0);
                    }
                },
                Some(x) => println!(" Unknown command '{}'. Try again", x),
                None => println!(" No command given. Try again"),
            };
        }

        Ok(())
    }
}
