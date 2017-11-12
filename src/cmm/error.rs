extern crate lalrpop_util;
use lalrpop_util::ParseError;

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

    pub fn get_line_with_off(&self, loc: usize) -> Option<(usize, usize)> {
        let mut count = 0;

        for (i, line) in self.lines.iter().enumerate() {
            if loc < (count + line.len()) {
                return Some((i, loc - count - 1))
            }

            count += line.len();
        }

        None
    }

    pub fn print_with_line(&self, header: &'a str, loc: usize, msg: &'a str) {
        let (i, off) = self.get_line_with_off(loc).unwrap();
        let line = self.lines.get(i).unwrap();

        println!("{} on line {}", header, i);
        println!("{}", line);
        println!("{}^", String::from_utf8(vec![b' '; off]).unwrap());
        println!("{}", msg);
    }

    pub fn print_parse_error<'input>(&self, err: lalrpop_util::ParseError<usize, (usize, &'input str), ()>) {
        let (msgo, loc) = match err {
            ParseError::InvalidToken { location } =>
                (Some(format!("Invalid token")), location),
            ParseError::UnrecognizedToken { token: Some((loc, (_, tok), _)), expected: exp } =>
                (Some(format!("Unrecognized token {:?}. Expected either {:?}", tok, exp)), loc),
            ParseError::ExtraToken { token: (loc, tok, _) } =>
                (Some(format!("Extra token {:?}", tok)), loc),
            _ => (None, 0),
        };

        let (i, off) = self.get_line_with_off(loc).unwrap();
        let line = self.lines.get(i).unwrap();

        println!("Syntax Error : line {} ({})", i, self.filename);

        if let Some(msg) = msgo {
            println!("{}", line);
            println!("{}^", String::from_utf8(vec![b' '; off]).unwrap());
            println!("{}", msg);
        }
    }
}
