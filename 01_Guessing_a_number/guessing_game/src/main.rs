use std::io;

fn main(){
    println!("Guessing a Number");
    println!("Please input a number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Readline!");

    println!("you guessed:{}",guess)
}