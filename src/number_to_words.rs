/*!
Reusable library that contains functions for **converting a numbers into their word forms** (e.g. ```123``` => ```"one hundred twenty-three"```)


Custom level_words are supported for use instead of the default place values found in ```DEFAULT_LEVEL_WORDS```
*/

/**
**Default place value** strings that cover all values up to ```u64::MAX``` (used if ```None``` is passed in to any of the functions as the level_words argument)
*/
const DEFAULT_LEVEL_WORDS: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

/**
Converts an **```i64``` to word form**


Internally uses ```unsigned_number_to_words``` (conversion safety is that the upper bound of a ```u64``` is at least that of a ```i64``` so there will be no overflow)
*/
pub fn signed_number_to_words(number: i64, level_words: Option<Vec<&str>>) -> String {
    if number >= 0 {
        unsigned_number_to_words(number as u64, level_words)
    } else {
        let unsigned_string = unsigned_number_to_words((-number) as u64, level_words);
        format!("negative {}", unsigned_string)
    }
}

/**
Converts a **```u64``` to word form**
*/
pub fn unsigned_number_to_words(number: u64, level_words: Option<Vec<&str>>) -> String {
    if number == 0 {
        "zero".to_string()
    } else {
        let number_string = number.to_string();
        let level_words = match level_words {
            Some(level_words) => level_words,
            None => DEFAULT_LEVEL_WORDS.to_vec(),
        };

        let mut groups = vec![];

        let mut number_characters = number_string.chars();
        let mut offset = 0;

        while offset <= number_string.len() {
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
                let group_level = level_words[index];
                let group_string = format!("{} {}", group_word, group_level);

                results.push(group_string);
            }
        }

        results.reverse();

        results.join(" ").trim_end().to_string()
    }
}

/**
**Internal helper function** than returns correct output for inputs less than 1000
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
**Basic test suite** for ```number_to_words``` testing the ```signed_number_to_words``` and ```unsigned_number_to_words``` functions
*/
#[cfg(test)]
mod tests {
    /**
    Tests a **positive input** and the ```unsigned_number_to_words``` function
    */
    #[test]
    fn test_positive() {
        assert_eq!(super::unsigned_number_to_words(1234567890, None), "one billion two hundred thirty-four million five hundred sixty-seven thousand eight hundred ninety");
    }

    /**
    Tests a **negative input** and the ```signed_number_to_words``` function
    */
    #[test]
    fn test_negative() {
        assert_eq!(super::signed_number_to_words(-1234567890, None), "negative one billion two hundred thirty-four million five hundred sixty-seven thousand eight hundred ninety");
    }
}
