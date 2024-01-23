use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

fn main() {
    
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Variable Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small...".blue()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!!!".green());
                break;
            },
        }
    }

}
