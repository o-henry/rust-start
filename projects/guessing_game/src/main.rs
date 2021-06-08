extern crate rand;
//       __ The io library -> comes from the standard library (which is known as std)
//     __ :: syntax -> indicates that io is an associated function of the std type.
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input ypur guess.");

    //  ___ storing values with mutable(mut) variables
    let mut guess = String::new();
                       // _ &: The & indicates that this argument is a reference, immutable
    //          _________ read_line : return io::Result -> Ok | Err
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        
    println!("You guessed: {}", guess);
}


