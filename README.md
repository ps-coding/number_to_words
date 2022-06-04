# number_to_words
This is a **library that converts a number to words**. It is written completely in rust and does not require any third-party dependencies.

## Examples:
- unsigned_number_to_words(0) => "zero"
- unsigned_number_to_words(12) => "twelve"
- unsigned_number_to_words(1993) => "one thousand nine hundred ninety-three"
- signed_number_to_words(-2234444) => "negative two million two hundred thirty-four thousand four hundred forty-four"

## Files
- *number_to_words.rs* is the *library*
- *main.rs* is just an *interactive REPL* using the library

## REPL
The main.rs file is a REPL that allows numbers to be entered and the word form to be viewed using the number_to_words library.
- *Clone* the repository
- Make sure you have *cargo* ([install rust](https://www.rust-lang.org/tools/install))
- *Run the following command* in the terminal while in the cloned directory
```shell
cargo run
```
