
// we need to bring the io input/output library into scope.
use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng;

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}!
    Now here are some numbers!");

    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
}
