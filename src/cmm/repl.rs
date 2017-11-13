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
        println!("map {:?}", map);
        Repl {
            enabled: enabled,
            skip: 0,
            map: map,
            last_line: 0,
        }
    }

    pub fn show<'input>(&mut self, stmt: CStmt<'input>) {
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

            // TODO: fix blank lines

            if self.skip > 0 {
                println!("-> skip {}", self.skip);
                self.skip -= 1
            } else {
                println!("-> run");
            }

            self.last_line = line;
        }
    }
}
