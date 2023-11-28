use std::{cmp, io};
// * import random from submodule

// extern crate rand;
use rand::Rng;

pub fn guess() {
    println!("Guess the number!");
    // * generate random number between 1 and 100
    let secret_number = Rng::gen_range(&mut rand::thread_rng(), 1..=100);

    // let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
