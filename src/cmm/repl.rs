use ast::CStmt;
use util;

#[derive(Clone)]
pub struct Repl {
    enabled: bool,
    skip: usize,
    map: Vec<usize>,
}

impl<'a> Repl {
    pub fn new(enabled: bool, program: &'a str) -> Repl {
        let lines: Vec<&'a str> = program.split('\n').collect();
        let map = lines.into_iter().map(|line| line.len() + 1).collect();
        println!("map {:?}", map);
        Repl {
            enabled: enabled,
            skip: 0,
            map: map
        }
    }

    pub fn show<'input>(&mut self, stmt: CStmt<'input>) {
        let loc = match stmt {
            CStmt::Decl((l, _), ..) => Some(l),
            _ => None,
        };
        let opt = match loc {
            Some(x) => util::line_from(x, &self.map),
            None => None
        };

        if let Some(line) = opt {
            println!("repl ({}/{}): {:?}", line, loc.unwrap(), stmt);

            if self.skip > 0 {
                println!("repl: skip {}", self.skip);
                self.skip -= 1
            } else {
                println!("repl: run");
            }
        }
    }
}
