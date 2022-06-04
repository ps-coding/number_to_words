/*!
A **REPL** to demonstrate the capabilities of the number_to_words library

This is ***not*** a part of the main library implementation
*/

mod number_to_words;

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
        } else {
            match input.parse::<i64>() {
                Ok(number) => {
                    let number_string = number_to_words::signed_number_to_words(number, None);

                    println!("Value: {}", number_string);
                }
                Err(_) => {
                    println!("Error: please enter a valid (u64) integer")
                }
            }
        }
    }
}
