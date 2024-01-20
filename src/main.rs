use rand::*;
use std::io;

fn main() {
    println!("**************************************");
    println!("\tWelcome to guessing game\t");
    println!("**************************************");
    println!("\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is:{secret_number}");
    println!("Please enter your guess");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline ");
        // println!("You guessed:{guess}");

        let guess = guess
            .trim()
            .parse::<i32>()
            .expect("Please enter some number");

        if guess > secret_number {
            println!("Guessed number is high ")
        } else if guess < secret_number {
            println!("Guessed number is low")
        } else {
            println!("You guessed it perfectly");
            break;
        }
    }
}
