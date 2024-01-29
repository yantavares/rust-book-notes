use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!(
                "Guess value must be between 1 and 100, got {}.",
                value
            ));
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess_str = String::new();

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: i32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        let guess = match Guess::new(guess) {
            Ok(g) => g,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small...".blue()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!!!".green());
                break;
            }
        }
    }
}
