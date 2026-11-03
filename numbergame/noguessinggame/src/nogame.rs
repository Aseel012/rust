use std::io;
use rand::Rng;

fn main() {
    let no = rand::rng().random_range(0..=100);

    println!("Guess a number between 1 to 100");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap(); 

        let guess: i32 = input.trim().parse().unwrap();

        if guess > no {
            println!("too high");
        }
        else if guess < no {
            println!("too low");
        }
        else {
            println!("you won");
            break;
        }
    }
}                                                 