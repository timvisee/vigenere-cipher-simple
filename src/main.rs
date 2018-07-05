#![feature(euclidean_division)]

extern crate rayon;

mod words;

use std::fs::read_to_string;
use rayon::prelude::*;
use words::Words;

fn main() {
    // Read the ciphertext, and a dictionary
    let ciphertext = read_to_string("ciphertext.txt").expect("failed to read ciphertext.txt");
    let dict = read_to_string("dictionary.txt").expect("failed to read dictionary.txt");

    // Collect uppercase dictionary words in a vector
    let mut dict: Vec<String> = dict
        .split_terminator(char::is_whitespace)
        .map(|word| word.to_uppercase())
        .collect();
    dict.sort_unstable();

    // Brute force dictionary keys without special characters concurrently
    dict.par_iter()
        .filter(|word| word.chars().all(|c| c.is_ascii_alphabetic()))
        .for_each(|key| {
            // Build a vector containing the shift values
            let shifts = key.chars()
                .map(|c| c as i16 - 'A' as i16)
                .collect();

            // Build a decryption iterator, compare some words at a dictionary, report matches
            if shift_iterator(&ciphertext, &shifts)
                .words()
                .take(8)
                .all(|ref word| dict.binary_search(word).is_ok())
            {
                // Decrypt only the first 250 characters to report
                let text: String = shift_iterator(&ciphertext, &shifts)
                    .take(250)
                    .collect();

                println!("==================");
                println!("FOUND POSSIBLE KEY!");
                println!("KEY: {} {:?}", key, shifts);
                println!("TEXT: {}... (truncated)", text);
                println!("==================");
            }
        });
}

/// Build an iterator to shift the given input string as defined by the `shifts` vector.
/// The returned iterator is lazy in the sense that only the characters being consumed will be
/// shifted as this is great for performance.
///
/// The sequence of shifts will be used to shift each alphabetic character each with their own next
/// shift value. The list of shifts is cycled.
///
/// The defined shift values will be subtracted from characters as this function should be used for
/// decryption. If a shift value of 2 is given, the character `C` will be shifted to `A`.
fn shift_iterator<'a>(ciphertext: &'a str, shifts: &'a Vec<i16>)
    -> impl Iterator<Item = char> + 'a
{
    // Cycle the shifting sequence and build the shifting iterator
    let mut shifts = shifts.iter().cycle();
    ciphertext.chars()
        .map(move |c| {
            if c.is_ascii_alphabetic() {
                let s = shifts.next().unwrap();
                ((c as i16 - 'A' as i16 - s).mod_euc(26) + 'A' as i16) as u8 as char
            } else {
                c
            }
        })
}
