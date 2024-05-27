use regex::Regex;
use std::env;
use std::fs;
use std::process;

mod args;

fn main() {
    let args_raw: Vec<String> = env::args().collect();

    let args = args::Args::new(args_raw).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("Searching for {}", args.match_expression);
    println!("In File {}", args.file_path);

    let file_contents = fs::read_to_string(&args.file_path).unwrap_or_else(|err| {
        println!("Could not open file at {}: {}", &args.file_path, err);
        process::exit(1);
    });

    let regex_parsed = Regex::new(&args.match_expression).unwrap_or_else(|err| {
        println!(
            "Could not process expression {}: {}",
            &args.match_expression, err
        );
        process::exit(1);
    });

    let found: Vec<&str> = regex_parsed
        .find_iter(&file_contents)
        .map(|m| m.as_str())
        .collect();

    dbg!(found);
}
