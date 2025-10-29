
use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Mutable so value can be stored here.

        io::stdin() 
            .read_line(&mut guess) // Returns Result. 2 Variants of Result: Ok(_) or Err(_)
            .expect("Failed to read line"); // Expect does. Ok(Val) => Val, Err(_) => println!(msg="Failed to ...")

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                            println!("You win!"); 
                            break;
                            }
        }
    }
}
