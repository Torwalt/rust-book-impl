use std::env;
use std::fs;
use std::process;

mod args;
mod search;

fn main() {
    let args_raw: Vec<String> = env::args().collect();

    let args = args::Args::new(args_raw).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("Searching for {}", args.word_search);
    println!("In File {}", args.file_path);

    let file_contents = fs::read_to_string(&args.file_path).unwrap_or_else(|err| {
        println!("Could not open file at {}: {}", &args.file_path, err);
        process::exit(1);
    });

    let found = search::lines_containing_search(&args.word_search, &file_contents);
    dbg!(found);
}
