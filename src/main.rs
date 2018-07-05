#![feature(euclidean_division)]

extern crate rayon;

use std::fs::read_to_string;

use rayon::prelude::*;

fn main() {
    // Read the ciphertext, and a dictionary
    let cipher = read_to_string("ciphertext.txt").expect("failed to read ciphertext.txt");
    let dict = read_to_string("dictionary.txt").expect("failed to read dictionary.txt");

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

    // Brute force each dictionary word as key
    dict.par_iter()
        .for_each(|key| {
            // Build a vector containing the shift values
            let shifts = key.chars()
                .map(|c| c as i16 - 'A' as i16)
                .collect();

            // Shift the ciphertext
            let output: String = shift_input(&cipher, &shifts);

            // Test whether this may be a match by comparing against the dictionary
            let is_match = output
                .split_terminator(|c: char| c.is_whitespace())
                .filter(|word| word.len() >= 5)
                .take(8)
                .filter(|word| dict.binary_search(word).is_ok())
                .count() >= 3;

            // Report
            if is_match {
                println!("==================");
                println!("FOUND POSSIBLE MATCH!");
                println!("KEY: {} {:?}", key, shifts);
                println!("TEXT: {}...", output.chars().take(170).collect::<String>());
                println!("==================");
            }
        });

    println!("DONE");
}

/// Shift the given input, by the given indices.
/// Each alpha character increases the current index by one,
/// clipping at the number of shifts given.
fn shift_input(cipher: &str, shifts: &Vec<i16>) -> String {
    let mut i = 0;
    cipher.chars()
        .map(|mut c| {
            // Shift alphabetic characters as specified, increase the shifting index
            if c.is_alphabetic() {
                c = ((c as i16 - 'A' as i16 - shifts[i]).mod_euc(26) + 'A' as i16) as u8 as char;
                i = (i + 1) % shifts.len();
            }

            c
        })
        .collect()
}
