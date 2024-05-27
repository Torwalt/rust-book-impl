use std::{fmt::Display, result::Result};

const FILE_PATH: &str = "-file_path";
const MATCH_EXPRESSION: &str = "-expression";

#[derive(Debug, PartialEq)]
pub struct Args {
    pub fp: String,
    pub me: String,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    FilePathMissingErr,
    ExpressionMissingErr,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FilePathMissingErr => write!(f, "No filepath given."),
            Error::ExpressionMissingErr => write!(f, "No regex given."),
        }
    }
}

impl Args {
    pub fn new(raw: Vec<String>) -> Result<Args, Error> {
        let args = Args::build(raw);
        if args.fp.is_empty() {
            return Err(Error::FilePathMissingErr);
        }

        if args.me.is_empty() {
            return Err(Error::ExpressionMissingErr);
        }

        return Ok(args);
    }

    fn build(raw: Vec<String>) -> Args {
        let mut args = Args {
            fp: String::new(),
            me: String::new(),
        };

        let mut arg_iter = raw.iter();
        loop {
            let maybe_arg = arg_iter.next();
            match maybe_arg {
                None => return args,
                Some(arg) => match arg.as_str() {
                    FILE_PATH => {
                        let v = arg_iter.next();
                        args.fp = v.unwrap_or(&String::new()).clone();
                    }

                    MATCH_EXPRESSION => {
                        let v = arg_iter.next();
                        args.me = v.unwrap_or(&String::new()).clone();
                    }

                    _ => {
                        continue;
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_args() {
        let default_expression = String::from("*");
        let file_path = String::from("./path/to/file.txt");
        let correct_input: Vec<String> = vec![
            String::from("target/debug/minigrep"),
            String::from(FILE_PATH),
            file_path.clone(),
            String::from(MATCH_EXPRESSION),
            default_expression.clone(),
        ];
        let args = Args::new(correct_input);
        let expected_args = Args {
            fp: file_path.clone(),
            me: default_expression.clone(),
        };

        assert_eq!(args.unwrap(), expected_args);

        let different_order_arg_input: Vec<String> = vec![
            String::from(MATCH_EXPRESSION),
            default_expression.clone(),
            String::from("something else"),
            String::from("-file_path"),
            file_path.clone(),
            String::from("target/debug/minigrep"),
        ];
        let args = Args::new(different_order_arg_input);
        let expected_args = Args {
            fp: file_path.clone(),
            me: default_expression.clone(),
        };

        assert_eq!(args.unwrap(), expected_args);

        // Err cases.

        let missing_arg_input: Vec<String> = vec![String::from("target/debug/minigrep")];
        let maybe_args = Args::new(missing_arg_input);

        assert_eq!(maybe_args, Err(Error::FilePathMissingErr));

        let missing_arg_input: Vec<String> = vec![
            String::from("target/debug/minigrep"),
            String::from(FILE_PATH),
            file_path.clone(),
        ];
        let maybe_args = Args::new(missing_arg_input);

        assert_eq!(maybe_args, Err(Error::ExpressionMissingErr));
    }
}
