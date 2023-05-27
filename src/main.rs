/*!
A **REPL** to demonstrate the capabilities of the number_to_words library

This is ***not*** a part of the main library implementation
*/

use number_to_words as lib;

fn main() {
    loop {
        print!("\nInput: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" || input == "exit" || input == "stop" || input == "bye" {
            println!("Status: exiting...");
            break;
        } else if input.chars().nth(0) == Some('-') {
            match input.parse::<i64>() {
                Ok(number) => {
                    let number_string = lib::signed_number_to_words(number, None);

                    println!("Value: {}", number_string);
                }
                Err(_) => {
                    println!("Error: please enter a valid (i64) integer")
                }
            }
        } else {
            match input.parse::<u64>() {
                Ok(number) => {
                    let number_string = lib::unsigned_number_to_words(number, None);

                    println!("Value: {}", number_string);
                }
                Err(_) => {
                    println!("Error: please enter a valid (u64) integer")
                }
            }
        }
    }
}
