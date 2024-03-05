use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);;

    loop {
        let mut guess = String::new();

        println!("Guess the number: ");
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line");

        let guess: u32 = match guess.trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
    }
}
