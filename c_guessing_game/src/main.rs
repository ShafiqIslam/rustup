use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("\n\nGuess the number!\n\n");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please guess a number between 1 and 100: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
