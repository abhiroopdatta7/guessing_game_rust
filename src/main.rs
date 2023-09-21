use std::{io, cmp::Ordering};
use colored::*;
use rand::Rng;

struct GameData {
    secret: u32,
    low: u32,
    high: u32,
    attempt: u32,
}

impl GameData {
    fn secret(low: u32, high: u32) -> GameData{
        println!("Generating secret to guess, low: {}, high {}", low, high);
        let secret = rand::thread_rng().gen_range(low, high);
        GameData {
            secret,
            low : low - 1,
            high : high - 1,
            attempt: 0,
        }
    }

    fn increase_attempt(&mut self) {
        self.attempt += 1;
    }

    fn updt_low(&mut self, low: u32) {
        self.low = low;
    }

    fn updt_high(&mut self, high: u32) {
        self.high = high;
    }

    fn play(&mut self) {
        let emoji_partyingface = '\u{1F973}';

        loop {
            println!("Attempt: {}, enter you guess between {} and {}: ", self.attempt , self.low, self.high);
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

            match guess.cmp(&self.secret) {
                Ordering::Less => {
                    print!("{}","Too Small, ".yellow());
                    self.updt_low(guess);
                    self.increase_attempt();
                },
                Ordering::Greater=> {
                    print!("{}","Too Big, ".red());
                    self.updt_high(guess);
                    self.increase_attempt();
                },
                Ordering::Equal=> {
                    println!("{} {} {}", emoji_partyingface, "You win, Correct ans is: ".green(), guess);
                    break;        
                },
            }
        };
    }

}


fn main() {
    let mut game_data = GameData::secret(1, 101);

    game_data.play();
    
}
