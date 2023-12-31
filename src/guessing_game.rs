use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

const EMOJI_PARTYINGFACE: char = '\u{1F973}';

pub struct Game {
    secret: u32,
    low: u32,
    high: u32,
    attempt: u32,
}

impl Game {
    #[cfg(debug_assertions)]
    fn print_secret(&self) {
        println!("[[DEBUG]] The secret is: {}", self.secret);
    }

    #[cfg(not(debug_assertions))]
    fn print_secret(&self) {}

    fn increase_attempt(&mut self) {
        self.attempt += 1;
    }

    fn updt_low(&mut self, low: u32) {
        self.low = low;
    }

    fn updt_high(&mut self, high: u32) {
        self.high = high;
    }

    pub fn init(low_i: u32, high_i: u32) -> Game {
        let secret = rand::thread_rng().gen_range(low_i, high_i);
        let low = low_i - 1;
        let high = high_i - 1;
        println!("Generating secret to guess, between {} and {}", low, high);

        Game {
            secret,
            low,
            high,
            attempt: 0,
        }
    }

    pub fn play(&mut self) {
        self.print_secret();

        loop {
            println!(
                "Attempt: {}, enter you guess between {} and {}: ",
                self.attempt, self.low, self.high
            );
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read guess.");

            let guess: u32 = match guess.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    print!("Please enter a number, ");
                    continue;
                }
            };

            match guess.cmp(&self.secret) {
                Ordering::Less => {
                    print!("{}", "Too Small, ".yellow());
                    self.updt_low(guess);
                    self.increase_attempt();
                }
                Ordering::Greater => {
                    print!("{}", "Too Big, ".red());
                    self.updt_high(guess);
                    self.increase_attempt();
                }
                Ordering::Equal => {
                    println!(
                        "{} {} {}",
                        EMOJI_PARTYINGFACE,
                        "You win, Correct ans is: ".green(),
                        guess
                    );
                    break;
                }
            }
        }
    }
}
