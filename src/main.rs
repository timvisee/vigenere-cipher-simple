extern crate rayon;

use std::fs::read_to_string;

use rayon::prelude::*;

fn main() {
    // Read the cipher text, and a dictionary
    let cipher = read_to_string("challenge.txt")
        .expect("failed to read cipher text at challenge.txt");
    let mut dict = read_to_string("/usr/share/dict/american-english-insane")
        .expect("failed to read dictionary at /usr/share/dict/american-english-insane");
    dict += &read_to_string("/usr/share/dict/british-english-insane")
        .expect("failed to read dictionary at /usr/share/dict/british-english-insane");

    // Collect uppercase dictionary words in a vector
    let mut dict: Vec<String> = dict
        .split_terminator(|c: char| c.is_whitespace())
        .filter(|word| word.chars().all(|c| c.is_alphabetic()))
        .map(|word| word.to_uppercase())
        .collect();
    dict.sort_unstable();
    dict.dedup();
    let dict: Vec<&str> = dict
        .iter()
        .map(|s| s.as_str())
        .collect();

    dict.par_iter()
        .for_each(|key| {
            // Build the shifting vector
            let shifts = key.chars()
                .map(|c| c as u8 - 'A' as u8)
                .collect();

            // Shift
            let output: String = shift_input(&cipher, &shifts);

            // Loop through the words
            let found = output
                .split_terminator(|c: char| c.is_whitespace())
                .filter(|word| word.len() >= 5)
                .take(8)
                //.inspect(|word| println!("FOUND WORD: {}", word))
                .filter(|word| dict.binary_search(word).is_ok())
                //.inspect(|word| println!("FOUND WORD: {}", word))
                .count() >= 3;

            if found {
                println!("==================");
                println!("HORRAAYYYY!!!!");
                println!("KEY: {}", key);
                println!("SHIFTS: {:?}", shifts);
                //println!("TRY: {:?}", &output[0..upto]);
                println!("TRY: {:?}", output);
                println!("==================");
            }
        });

    println!("DONE");
}

/// Shift the given input, by the given indices.
/// Each alpha character increases the current index by one,
/// clipping at the number of shifts given.
fn shift_input(cipher: &str, shifts: &Vec<u8>) -> String {
    // Define how to shift each 
    let mut i = 0;
    cipher
        .chars()
        .map(|c| {
            // Define the indiced char
            let t = (i, c);

            // Increase the shift index
            if c.is_alphabetic() {
                i = (i + 1) % shifts.len();
            }

            t
        })
        .map(|(i, c)| {
            // Only actually shift alpha characters
            if !c.is_alphabetic() {
                return c;
            }

            // Shift by the defined index
            (((c as u8 - 'A' as u8 + shifts[i]) % 26) + 'A' as u8) as char
        })
        .collect()
}
