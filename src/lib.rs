/*!
Reusable library that contains functions for **converting a numbers into their word forms** (e.g. ```123``` => ```"one hundred twenty-three"```)


Custom level_words are supported for use instead of the default place values found in ```DEFAULT_LEVEL_WORDS```
*/

/**
**Default place value** strings that cover all values up to ```u64::MAX``` (used if ```None``` is passed in to any of the functions as the level_words argument)
*/
const DEFAULT_LEVEL_WORDS: [&str; 11] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
];

/**
Converts an integer into its word form, taking the integer as a string slice of arbitrary length and an optional array slice for the place values ("thousand", "million", etc.)
*/
pub fn number_to_words(number: &str, level_words: Option<&[&str]>) -> Option<String> {
    let number = number.trim();

    if number.is_empty() {
        return None;
    }

    if !number.starts_with('-') {
        unsigned_number_to_words(number, level_words)
    } else {
        let mut chars = number.chars();
        chars.next();
        let number = chars.as_str();

        let unsigned_string = unsigned_number_to_words(number, level_words)?;
        Some(format!("negative {}", unsigned_string))
    }
}

/**
**Internal helper function** that returns correct output for all positive inputs
*/
fn unsigned_number_to_words(number: &str, level_words: Option<&[&str]>) -> Option<String> {
    if !number.chars().all(char::is_numeric) {
        return None;
    }

    if number == "0" {
        Some("zero".to_string())
    } else {
        let level_words = level_words.unwrap_or(&DEFAULT_LEVEL_WORDS);

        let mut groups = vec![];

        let mut number_characters = number.chars();
        let mut offset = 0;

        while offset <= number.len() {
            let third_digit_number = number_characters
                .next_back()
                .unwrap_or('0')
                .to_string()
                .parse::<u64>()
                .unwrap();
            let second_digit_number = number_characters
                .next_back()
                .unwrap_or('0')
                .to_string()
                .parse::<u64>()
                .unwrap();
            let first_digit_number = number_characters
                .next_back()
                .unwrap_or('0')
                .to_string()
                .parse::<u64>()
                .unwrap();

            let term = (100 * first_digit_number) + (10 * second_digit_number) + third_digit_number;

            groups.push(term);

            offset += 3
        }

        let mut results = vec![];

        for (index, group) in groups.iter().enumerate() {
            let group_word = group_number_to_words(*group);

            if !group_word.is_empty() {
                let mut group_level = "";

                if index > 0 {
                    group_level = level_words.get(index - 1)?;
                }

                let group_string = format!("{} {}", group_word, group_level);

                results.push(group_string);
            }
        }

        results.reverse();

        Some(results.join(" ").trim_end().to_string())
    }
}

/**
**Internal helper function** that returns correct output for inputs less than 1000
*/
fn group_number_to_words(number: u64) -> String {
    if number < 20 {
        (match number {
            0 => "",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => unreachable!("The value must be less than 20 due to previous if check."),
        })
        .trim()
        .to_string()
    } else if number < 100 {
        let first_digit_number = number
            .to_string()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as u64;
        let first_part = multiple_of_ten_number_to_words(first_digit_number * 10);

        let last_digit_number = number - (10 * first_digit_number);
        let last_part = group_number_to_words(last_digit_number);

        if !last_part.is_empty() {
            format!("{}-{}", first_part, last_part).trim().to_string()
        } else {
            first_part.trim().to_string()
        }
    } else if number < 1000 {
        let first_digit_number = number
            .to_string()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as u64;
        let first_part = format!("{} hundred", group_number_to_words(first_digit_number));

        let last_two_digits_number = number - (100 * first_digit_number);

        let last_part = group_number_to_words(last_two_digits_number);

        format!("{} {}", first_part, last_part).trim().to_string()
    } else {
        panic!(
            "Internal error - Invalid number passed to group_number_to_words: {}.",
            number
        )
    }
}

/**
**Internal helper function** that returns the word for a multiple of ten greater than 10 and less than 100
*/
fn multiple_of_ten_number_to_words(number: u64) -> String {
    (match number {
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => {
            panic!(
                "Internal Error - Invalid number passed to multiple_of_ten_number_to_words: {}.",
                number
            );
        }
    })
    .to_string()
}

/**
**Basic test suite** for ```number_to_words```
*/
#[cfg(test)]
mod tests {
    use super::*;
    /**
    Tests a **positive input**
    */
    #[test]
    fn test_positive() {
        assert_eq!(number_to_words("1234567890", None), Some("one billion two hundred thirty-four million five hundred sixty-seven thousand eight hundred ninety".to_string()));
    }

    /**
    Tests a **negative input**
    */
    #[test]
    fn test_negative() {
        assert_eq!(number_to_words("-1234567890", None), Some("negative one billion two hundred thirty-four million five hundred sixty-seven thousand eight hundred ninety".to_string()));
    }
}
