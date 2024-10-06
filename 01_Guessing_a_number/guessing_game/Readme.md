# Guessing game

## Import Statement:

`use std::io;`: This line imports the io module from the standard library. The io module provides functions for input/output operations, such as reading from the console and writing to the console.
Main Function:

`fn main() { ... }`: This is the entry point of the Rust program. The main function is where the execution of the program begins.
Printing Messages:

`println!("Guess the Number");`: This line prints the message "Guess the Number" to the console.
`println!("Please input a number");`: This line prints the message "Please input a number" to the console, prompting the user to enter a guess.

## Creating a String:

`let mut guess = String::new();`: This line declares a mutable variable named guess of type String. The `String::new()` function creates an empty string. The mut keyword indicates that the value of the guess variable can be modified later in the code.

## Reading User Input:

`io::stdin().read_line(&mut guess).expect("Failed to read line");`: This line reads a line of text from the standard input (the console) and stores it in the guess variable.

`io::stdin()`: This part obtains a handle to the standard input stream.

`.read_line(&mut guess)`: This part calls the read_line method on the standard input handle, passing a mutable reference to the guess variable as an argument. The read_line method reads a line of text from the input and appends it to the guess string.

`.expect("Failed to read line")`: This part handles potential errors that might occur during the reading process. If an error occurs, the expect method panics, causing the program to terminate with an error message.

## Printing the Guessed Number:

`println!("you guessed: {}", guess);`: This line prints the message "you guessed: " followed by the value of the guess variable to the console.
