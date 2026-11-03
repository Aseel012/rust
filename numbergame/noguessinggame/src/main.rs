use std::io;
use rand::Rng;

fn main() {
    let secret = rand::rng().random_range(1..=100);

    println!("Guess a number between 1 and 100:");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let guess: i32 = input.trim().parse().unwrap();

        if guess < secret {
            println!("Too low");
        } else if guess > secret {
            println!("Too high");
        } else {
            println!("You won!");
            break;
        }
    }
}