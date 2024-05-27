use std::env;

static FILE_PATH: &str = "-file_path";

#[derive(Debug, PartialEq)]
struct Args {
    fp: String,
}

impl Args {
    fn new(raw: Vec<String>) -> Args {
        let mut args = Args { fp: String::new() };

        let mut arg_iter = raw.iter();
        loop {
            let x = arg_iter.next();
            match x {
                None => return args,
                Some(x) => {
                    if x == FILE_PATH {
                        let v = arg_iter.next();
                        args.fp = v.unwrap_or(&String::new()).clone();
                    }
                }
            }
        }
    }
}

fn main() {
    let args_raw: Vec<String> = env::args().collect();

    let args = Args::new(args_raw);

    println!("{:?}", args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_args() {
        let file_path = String::from("./path/to/file.txt");
        let correct_input: Vec<String> = vec![
            String::from("target/debug/minigrep"),
            String::from("-file_path"),
            file_path.clone(),
        ];
        let args = Args::new(correct_input);
        let expected_args = Args {
            fp: file_path.clone(),
        };

        assert_eq!(args, expected_args);

        let missing_arg_input: Vec<String> = vec![
            String::from("target/debug/minigrep"),
            String::from("-file_path"),
        ];
        let args = Args::new(missing_arg_input);
        let expected_args = Args { fp: String::new() };

        assert_eq!(args, expected_args);

        let no_arg_input: Vec<String> = vec![];
        let args = Args::new(no_arg_input);
        let expected_args = Args { fp: String::new() };

        assert_eq!(args, expected_args);

        let different_order_arg_input: Vec<String> = vec![
            String::from("something else"),
            String::from("-file_path"),
            file_path.clone(),
            String::from("target/debug/minigrep"),
        ];
        let args = Args::new(different_order_arg_input);
        let expected_args = Args {
            fp: file_path.clone(),
        };

        assert_eq!(args, expected_args);
    }
}
