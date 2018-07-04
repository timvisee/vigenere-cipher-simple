use std::fs::read_to_string;

fn main() {
    // Read the cipher text, and a dictionary
    let cipher = read_to_string("challenge.txt")
        .expect("failed to read cipher text at challenge.txt");
    let dict = read_to_string("/usr/share/dict/words")
        .expect("failed to read dictionary at /usr/share/dict/words");

    // Collect uppercase dictionary words in a vector
    let dict: Vec<String> = dict
        .split_terminator("\n")
        .map(|word| word.trim().to_uppercase())
        .collect();
    let mut dict: Vec<&str> = dict
        .iter()
        .map(|s| s.as_str())
        .collect();
    dict.sort_unstable();

    // Get the index of the last character of the 4th word
    let upto = cipher
    .chars()
        .enumerate()
        .filter(|(_, c)| *c == ' ')
        .skip(3) // +1
        .next()
        .expect("cannot find counted word")
        .0;

    // Brute force, increase the number of chiper indices each time
    for s in 2..100 {
        let mut shifts = vec![0u8; s];

        // Tell we're attempting a given shift
        println!("ATTEMPTING SHIFTS: {}", s);

        shift_combinations(&mut shifts, 0, |shifts| {
            // Shift
            let output: String = shift_input(&cipher, &shifts);

            // Loop through the words
            let found = output
                .split_terminator(|c| c == ' ' || c == '\n')
                .filter(|word| word.len() > 4)
                .take(2)
                .filter(|word| dict.binary_search(word).is_ok())
                .inspect(|word| println!("FOUND WORD: {}", word))
                .count() >= 2;

            if found {
                println!("TRY: {:?}", &output[0..upto]);
            }
        });
    }
}

/// Generate all possible shift combinations in the given `shifts` vector.
/// The current shifting index is specified with `i` used for recursion.
/// Therefore `0` should be specified for `i` when calling with to generate
/// combinations.
/// The closure `try` is invoked with each combination.
fn shift_combinations<F>(shifts: &mut Vec<u8>, i: usize, try: F)
    where F: Fn(&Vec<u8>) + Copy
{
    // Loop through all shift combinations for the current shift index
    for amount in 0..26 {
        shifts[i] = amount;

        // Walk through remaining index combinations
        if i < shifts.len() - 1 {
        	shift_combinations(shifts, i + 1, try);
        }

        // Try the combination when the last index is set
        if i == shifts.len() - 1 {
            try(shifts);
        }
    }
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
        .map(|(i, mut c)| {
            // Only actually shift alpha characters
            if !c.is_alphabetic() {
                return c;
            }

            // Shift by the defined index
            c = (((c as u8 - 'A' as u8 + shifts[i]) % 26) + 'A' as u8) as char;

            c
        })
        .collect()
}
