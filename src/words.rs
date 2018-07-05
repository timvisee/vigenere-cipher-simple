use std::iter::{Chain, FilterMap, Iterator, Scan};
use std::str::Chars;

/// The type of the scanning closure.
type S = fn(&mut String, char) -> Option<Option<String>>;

/// The type for the filter.
type F = fn(Option<String>) -> Option<String>;

/// An iterator to map a stream of characters into words.
/// This morphs an iterator of characters to an iterator of strings outputing words.
///
/// The iterator collects `char`s until a non-alphanumeric character is found,
/// which will make it yield a word as a `String`.
/// All non-alphanumeric characters are omitted from the output, this it will not contain any
/// spaces, commas or other characters.
///
/// See the `char::is_alphabetic()` method for a definition on what characters count as alphabetic.
// TODO: the current implementation fails on words like `I've`
pub trait Words {

    /// Morph an iterator of characters to an iterator of strings outputing words.
    ///
    /// The iterator collects `char`s until a non-alphanumeric character is found,
    /// which will make it yield a word as a `String`.
    /// All non-alphanumeric characters are omitted from the output, this it will not contain any
    /// spaces, commas or other characters.
    ///
    /// See the `char::is_alphabetic()` method for a definition on what characters count as alphabetic.
    fn words<'a>(self)
        -> FilterMap<Scan<Chain<Self, Chars<'a>>, String, S>, F>
        where
            Self: Sized + Iterator<Item = char>,
    {
        // A scan function to collect non-whitespace characters to yield strings at each whitespace
        let scan: S = |state: &mut String, c: char| {
            if c.is_alphabetic() {
                state.push(c);
            } else if !state.is_empty() {
                return Some(Some(state.split_off(0)));
            }

            Some(None)
        };

        // Define the word iterator, chain a space to ensure all words are collected
        self.chain(" ".chars())
            .scan(String::new(), scan)
            .filter_map(|word| word)
    }
}

impl<I: Iterator> Words for I { }
