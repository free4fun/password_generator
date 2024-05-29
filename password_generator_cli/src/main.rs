use std::io;
use pw_lib; // Add import statement for pw_lib crate

pub fn main() {
    let len :u128;
    let mut input = String::new();

    println!("Password Generator");
    println!("Enter password lenght in number. Default is 32: ");
    io::stdin().read_line(&mut input).expect("Error reading input");
    len = input.trim().parse().expect("Invalid input");
    let mut choice: String = String::new();
    println!("Choose chars to use:");
    println!("1: Only ASCII");
    println!("2: Not only ASCII");
    io::stdin().read_line(&mut choice).expect("Error reading input");
    let choice: String = choice.trim().to_string();
    let rnd_string: String = pw_lib::main(len, &choice);
    println!("{}", rnd_string);
}