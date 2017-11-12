extern crate lalrpop_util;
use lalrpop_util::ParseError;

pub enum CError {
    ParseError(String, usize),
    RuntimeError(String, usize),
    CheckerError(Vec<String>),  // TODO: loc
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
            _ => panic!("unknown parse error: {:?}", err),
        }
    }
}

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
        match err {
            CError::ParseError(..) => self.print_simple_err(err),
            CError::RuntimeError(..) => self.print_simple_err(err),
            CError::CheckerError(..) => self.print_list_err(err),
        }
    }

    fn print_list_err(&self, err: CError) {
        let (head, es) = match err {
            CError::CheckerError(es) => ("Checker error", es),
            _ => panic!("unexpected type of error")
        };

        println!("TODO: print checker err");
        println!("{} : ({})", head, self.filename);
        println!("{:?}", es);
    }

    fn print_simple_err(&self, err: CError) {
        let (head, msg, loc) = match err {
            CError::ParseError(msg, loc) => ("Syntax error", msg, loc),
            CError::RuntimeError(msg, loc) => ("Run-time error", msg, loc),
            _ => panic!("unexpected type of error")
        };

        let (i, off) = self.get_line_with_off(loc).unwrap();
        let line = self.lines.get(i).unwrap();

        println!("{} : line {} ({})", head, i + 1, self.filename);

        println!("{}", line);
        println!("{}^", String::from_utf8(vec![b' '; off]).unwrap());
        println!("{}", msg);
    }

    fn get_line_with_off(&self, loc: usize) -> Option<(usize, usize)> {
        let mut count = 0;

        for (i, line) in self.lines.iter().enumerate() {
            if loc < (count + line.len()) {
                return Some((i, loc - count - 1))
            }

            count += line.len();
        }

        None
    }
}
