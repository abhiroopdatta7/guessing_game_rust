use std::{io, cmp::Ordering};
use colored::*;
use rand::Rng;

fn main() {
    let mut low = 1;
    let mut high = 101;
    let mut attempt: u32 = 0;
    let emoji_partyingface = '\u{1F973}';

    let secret = rand::thread_rng().gen_range(low, high);
    low -= 1;
    high -= 1;
    println!("Generating secret to guess, low: {}, high {}", low, high);
    // println!("The secret is: {}", secret);
    
    loop {
        println!("Attempt: {}, enter you guess between {} and {}: ", attempt , low, high);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess.");

        let guess:u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                print!("Please enter a number, ");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => {
                print!("{}","Too Small, ".yellow());
                low = guess;
                attempt += 1;
            },
            Ordering::Greater=> {
                print!("{}","Too Big, ".red());
                high = guess;
                attempt += 1;
            },
            Ordering::Equal=> {
                println!("{} {} {}", emoji_partyingface, "You win, Correct ans is: ".green(), guess);
                break;        
            },
        }
    
    }
}
