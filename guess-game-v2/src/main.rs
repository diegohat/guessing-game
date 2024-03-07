use std::io;

use rand::{thread_rng, Rng};

fn select_random_word(fruits: &[&str]) -> String {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..fruits.len());
    String::from(fruits[index])
}

fn check_word(guess: &str, random_word: &str) -> bool {
    let trimmed_guess = guess.trim();
    // println!("Guessed: '{}'", trimmed_guess);
    // println!("Random: '{}'", random_word);
    if trimmed_guess == random_word {
        println!("You guessed the word!");
        false
    } else {
        println!("Wrong!");
        true
    }
}

fn main() {
    let fruits = [
        "apple",
        "banana",
        "orange",
        "pear",
        "grape",
        "pineapple",
        "watermelon",
        "strawberry",
        "blueberry",
        "kiwi",
    ];

    println!("Welcome to Guessing Game! Press ENTER to start...");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to start game.");

    println!("Generating a random word...");
    let random_word = select_random_word(&fruits);

    loop {
        println!("Enter a fruit name: ");
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read word.");

        if !check_word(&guess, &random_word) {
            break;
        } else {
            println!("Try again!");
        }
    }

    println!("Good game!!");
}
