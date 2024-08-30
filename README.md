# number_to_words

This is a **robust library that converts a number (or, more specifically, an integer) to words**. It is written completely in Rust and does not require any third-party dependencies. It allows the entering of custom place value words (e.g., "thousand", "million", etc.) to extend the algorithm to numbers of arbitrary length, and it accepts the number as a string input instead of as a Rust integer to enable the conversion of _arbitrarily large numbers_.

There is only one public function, `number_to_words`, which accepts two parameters: the integer as a string and an array slice of strings representing the place values ("thousand", "million", etc.). If the place values are not provided, the default ones are used, which go up to a decillion (far beyond the maximum size of a u64). The function returns an `Option<String>` with the converted number as a string.

## Applications

- Text to speech applications
- Financial applications (words often have to be used in financial documents with or in place of digits)
- Education applications
- More things I can't think of right now

## Examples

- `number_to_words("0", None).unwrap()` => `"zero"`
- `number_to_words("12", None).unwrap()`=>`"twelve"`
- `number_to_words("1993", None).unwrap()`=>`"one thousand nine hundred ninety-three"`
- `number_to_words("-2234444", None).unwrap()`=>`"negative two million two hundred thirty-four thousand four hundred forty-four"`

## Limitations

This library is very powerful, but there are a few things to be aware of.

- The library does not handle decimals. It only converts integers. No extra characters within the number are allowed (besides an optional negative sign `-` at the start).
- The library is based on the English system of number groupings (groups of three digits), so it may not work as expected with other numbering systems.

## Files

- _lib.rs_ is the _library_
- _main.rs_ is just an _interactive converter_ using the library

## Interactive Converter Usage

- _Clone_ the repository
- Make sure you have _cargo_ ([install rust](https://www.rust-lang.org/tools/install))
- _Run the following command_ in the terminal while in the cloned directory

```shell
cargo run
```

- Enter `quit` to exit the program
