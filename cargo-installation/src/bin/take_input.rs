use std::io;

fn main() {
    let mut input = String::new();

    
    println!("Enter Name:");
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim();

    println!("{}", n);
}