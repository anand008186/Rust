use rand::prelude::*;
use std;    
fn main() {
    let mut rng = rand::thread_rng();
    let secret: u8 = rng.gen_range(1..=100);
  loop {
    println!("Please input your guess number between 1 and 100");
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
    let guess: u8 = guess.trim().parse().expect("Please type a number between 1 and 100");
    match guess.cmp(&secret) {
        std::cmp::Ordering::Less => println!("You guessed number is less than the random number!"),
        std::cmp::Ordering::Greater => println!("You guessed number is greater than the random number!"),
        std::cmp::Ordering::Equal => {
            println!("You guessed the correct number!");
            break;
        },

    }
  }


}