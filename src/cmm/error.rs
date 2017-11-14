extern crate lalrpop_util;

use lalrpop_util::ParseError;

#[derive(Debug)]
pub enum CError {
    ParseError(String, usize),
    RuntimeError(String, usize),
    CheckerError(Vec<(String, Option<usize>)>),
    UnknownError(String),
}

impl<'input> CError {
    pub fn from_lalrpop(err: lalrpop_util::ParseError<usize, (usize, &'input str), ()>) -> CError {
        match err {
            ParseError::InvalidToken { location } =>
                CError::ParseError(format!("Invalid token"), location),
            ParseError::UnrecognizedToken { token: Some((loc, (_, tok), _)), expected: exp } =>
                CError::ParseError(format!("Unrecognized token {:?}. Expected either {:?}", tok, exp), loc),
            ParseError::ExtraToken { token: (loc, tok, _) } =>
                CError::ParseError(format!("Extra token {:?}", tok), loc),
            _ => CError::UnknownError(format!("unknown parse error: {:?}", err)),
        }
    }
}

// printer

pub struct ErrorPrinter {
    filename: String,
    lines: Vec<String>,
}

impl<'a> ErrorPrinter {
    pub fn new(filename: &'a str, prog: &'a str) -> ErrorPrinter {
        ErrorPrinter {
            filename: String::from(filename),
            lines: prog.split("\n").map(|x| String::from(x)).collect(),
        }
    }

    pub fn print_err(&self, err: CError) {
        let (head, es) = match err {
            CError::ParseError(msg, loc) => ("Syntax error", vec![(msg, Some(loc))]),
            CError::RuntimeError(msg, loc) => ("Run-time error", vec![(msg, Some(loc))]),
            CError::CheckerError(es) => ("Checker error", es),
            CError::UnknownError(msg) => ("Error", vec![(msg, None)]),
        };

        for (msg, loc) in es {
            match loc {
                Some(loc) => {
                    let (i, off) = self.get_line_with_off(loc).unwrap();
                    println!("{}: line {}:{} ({})", head, i + 1, off, self.filename);
                    self.print_at_loc(loc)
                },
                None => println!("{}: ({})", head, self.filename),

            };
            println!(" -> {}", msg);
        }
    }

    fn print_at_loc(&self, loc: usize) {
        let (i, off) = self.get_line_with_off(loc).unwrap();
        let line = self.lines.get(i).unwrap();
        println!(" | {}", line);
        println!(" | {}^", String::from_utf8(vec![b' '; off]).unwrap());
    }

    fn get_line_with_off(&self, loc: usize) -> Option<(usize, usize)> {
        let mut count = 0;

        for (i, line) in self.lines.iter().enumerate() {
            if loc < (count + line.len() + 1) {
                return Some((i, loc - count))
            }

            // +1 for newline char
            count += line.len() + 1;
        }

        None
    }
}
