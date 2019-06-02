extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secrete number is : {}", secret_number);

    loop {
        println!("Input your guess!! ↓");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // guessを文字列から数字にしてる。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(num) => {
                println!("{} !!", num);
                continue;
            }
        };

        println!("Your Guess is : {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("exact");
                break;
            }
        }
    }
}
