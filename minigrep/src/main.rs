use regex::Regex;
use std::env;
use std::fs;
use std::process;

mod args;

fn main() {
    let args_raw: Vec<String> = env::args().collect();

    let args = match args::Args::new(args_raw) {
        Ok(a) => a,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

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
