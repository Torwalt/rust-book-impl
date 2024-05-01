use crate::{pig::to_pig_latin, stats::median_and_mode};

mod pig;
mod stats;

fn main() {
    println!("Hello, world!");

    let (median, mode): (usize, usize) = median_and_mode(&vec![1, 9, 7, 4, 1, 6, 7, 1]);
    println!("Median is {}, mode is {}", median, mode);

    let piggy = to_pig_latin(String::from("My Name, is Alex."));
    println!("As piglatin: {}", piggy);
}

// TODO:
// Using a hash map and vectors, create a text interface to allow a user to add employee names to
// a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.
