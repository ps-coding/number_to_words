/*!
A **REPL** to demonstrate the capabilities of the number_to_words library

This is ***not*** a part of the main library implementation
*/

use number_to_words::number_to_words;

fn main() {
    loop {
        print!("\nInput: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        } else {
            println!(
                "Output: {}",
                number_to_words(input, None).unwrap_or(
                    "Error: Please enter a valid number. Enter 'quit' to exit.".to_string()
                )
            );
        }
    }
}
