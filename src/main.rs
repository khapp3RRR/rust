use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    println!("Please input the number");

    loop {
        let secret_number = rand::thread_rng().gen_range(1, 10);

        let mut guess: String = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You gueesed: {} System: {}", guess, secret_number);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Small".red()),
            Ordering::Greater => println!("{}", "Greater!".red()),
            Ordering::Equal =>  {
                println!("{}", "Greater".green());
                break;
            }
        }

    }


}
