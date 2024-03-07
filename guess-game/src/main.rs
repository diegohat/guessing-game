use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a valid number: ");

        let mut guessed_number = String::new();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read input.");

        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guessed_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big.".red()),
            Ordering::Less => println!("{}", "Too small.".red()),
        }
    }
}
