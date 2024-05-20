use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number from 1 - 100!");

    let secret = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess_parsed = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must number!");
                continue;
            }
        };

        let guess = Guess::new(guess_parsed);

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

struct Guess {
    val: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must have a value between 1 and 100.")
        }
        return Guess { val: value };
    }

    pub fn value(&self) -> i32 {
        return self.val;
    }
}
