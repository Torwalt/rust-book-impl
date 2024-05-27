use regex::Regex;
use std::env;
use std::fs;
use std::process;

const FILE_PATH: &str = "-file_path";
const MATCH_EXPRESSION: &str = "-expression";

#[derive(Debug, PartialEq)]
struct Args {
    fp: String,
    me: String,
}

impl Args {
    fn new(raw: Vec<String>) -> Args {
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

fn main() {
    let args_raw: Vec<String> = env::args().collect();

    let args = Args::new(args_raw);

    if args.fp == String::new() {
        println!("No filepath given.");
        process::exit(1);
    }

    if args.me == String::new() {
        println!("No regex given.");
        process::exit(1);
    }

    let file_contents = match fs::read_to_string(&args.fp) {
        Ok(file) => file,
        Err(err) => {
            println!("Could not open file at {}: {}", &args.fp, err);
            process::exit(1);
        }
    };

    let regex_parsed = match Regex::new(&args.me) {
        Ok(rg) => rg,
        Err(err) => {
            println!("Could not process expression {}: {}", &args.me, err);
            process::exit(1);
        }
    };

    let found: Vec<&str> = regex_parsed
        .find_iter(&file_contents)
        .map(|m| m.as_str())
        .collect();

    dbg!(found);
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

        assert_eq!(args, expected_args);

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

        // Err cases.

        assert_eq!(args, expected_args);

        let missing_arg_input: Vec<String> = vec![
            String::from("target/debug/minigrep"),
            String::from("-file_path"),
        ];
        let args = Args::new(missing_arg_input);
        let expected_args = Args {
            fp: String::new(),
            me: String::new(),
        };

        assert_eq!(args, expected_args);

        let no_arg_input: Vec<String> = vec![];
        let args = Args::new(no_arg_input);
        let expected_args = Args {
            fp: String::new(),
            me: String::new(),
        };

        assert_eq!(args, expected_args);
    }
}
