use std::io; //input and output library, comes from the standard library
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line"); 
    // read_line() returns a Result, enum, variable, 
    // that is Ok or Err. And this Result must be handled.
    //That is what .expect() is doing here. 

    println!("You guessed: {guess}");
}
