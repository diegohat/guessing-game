use std::io;

use rand::{thread_rng, Rng};


fn select_random_word(fruits: &[&str]) -> String {
    
    let mut rng = thread_rng();
    let index = rng.gen_range(0..fruits.len());
    String::from(fruits[index])
}


fn main() {
    let fruits = ["apple", "banana", "orange", "pear", "grape", "pineapple", "watermelon", "strawberry", "blueberry", "kiwi"];

    println!("Welcome to Guessing Game! Press ENTER to start...");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to start game.");
    // TODO - add a select topic option
    println!("Generating a random word...");
    let random_word = select_random_word(&fruits);

    println!("Enter a fruit name: ");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read word.");
}
