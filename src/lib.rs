#![doc(html_root_url = "https://docs.rs/phonetic/1/")]
//! phonetic is a library that allows you to transform [`&str`]s to [`Vec`]s of words
//! from phonetic alphabets.
//!
//! Usage:
//! ```
//! use phonetic::{Phonetic, Alphabet};
//! use std::str::FromStr;
//!
//! // Load a phonetic alphabet using the Alphabet enum
//! let phonetic = Phonetic::load(Alphabet::nato).unwrap();
//! let wordlist = phonetic.string_to_words("abc".to_string());
//! assert_eq!(wordlist, ["Alpha", "Bravo", "Charlie"]);
//!
//! // Load a phonetic alphabet using an &str
//! let phonetic = Phonetic::from_str("nato").unwrap();
//! let wordlist = phonetic.string_to_words("abc".to_string());
//! assert_eq!(wordlist, ["Alpha", "Bravo", "Charlie"]);
//! ```
//!
//! Supported alphabets can be found in the [`Alphabet`] struct

include!(concat!(env!("OUT_DIR"), "/alphabet_kinds.rs"));

use indexmap::IndexMap;
use rust_embed::RustEmbed;
use std::{cmp::Reverse, str::FromStr};
use substring::Substring;

#[derive(RustEmbed)]
#[folder = "alphabets"]
struct Asset;

// Struct representing an alphabet
#[derive(Debug)]
pub struct Phonetic {
    words: IndexMap<String, String>,
    max_ngram_len: usize,
}

// Error returned when an alphabet can't be found
#[derive(Debug)]
pub struct AlphabetNotFoundError {}

/// Struct that represents an Alphabet
impl Phonetic {
    /// Load an alphabet based on it's name
    /// ```
    /// use phonetic::{Phonetic, Alphabet};
    ///
    /// let phonetic = Phonetic::load(Alphabet::nato);
    ///
    /// assert_eq!(phonetic.is_ok(), true);
    /// ```
    pub fn load(name: Alphabet) -> Result<Phonetic, AlphabetNotFoundError> {
        // Load the alphabet from an embedded asset into a utf8 string
        let embedded_file_option = Asset::get(name.to_string().as_str());
        let embedded_file = match &embedded_file_option {
            Some(f) => f,
            None => {
                return Err(AlphabetNotFoundError {});
            }
        };
        let alphabet_string = String::from_utf8_lossy(&embedded_file.data).to_string();

        // Split the string, filter out empty lines and turn it into a HashMap<String, String>
        let words: IndexMap<String, String> = alphabet_string
            .split('\n')
            .filter(|x| !x.is_empty() && !x.starts_with('#')) // filter empty lines and comments
            .map(|x| {
                let n: Vec<String> = x.splitn(2, ' ').map(|x| x.to_string()).collect();
                (n[0].to_lowercase(), n[1].clone())
            })
            .collect();

        let mut prefixes: Vec<_> = words.keys().collect();
        prefixes.sort_by_key(|b| Reverse(b.len()));
        let max_ngram_len = prefixes[0].len();

        Ok(Phonetic {
            words,
            max_ngram_len,
        })
    }

    /// Validate if there's a mapping for the given alphabet
    /// ```
    /// use phonetic::Phonetic;
    ///
    /// let res = Phonetic::validate("nato");
    /// assert_eq!(res.is_ok(), true);
    ///
    /// let res = Phonetic::validate("nonexistent");
    /// assert_eq!(res.is_err(), true);
    ///
    /// ```
    pub fn validate(s: &str) -> Result<(), String> {
        match Alphabet::from_str(s) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unknown alphabet: {}", s)),
        }
    }

    /// List all available alphabets. This function returns a [`Vec`] of tuples
    /// containing the `(alphabet abreviation, long name)` (e.g. `("fr-BE", "French (Belgum)")`)
    /// ```
    /// use phonetic::Phonetic;
    ///
    /// let alphabets = Phonetic::list();
    /// assert!(alphabets.len() > 0);
    /// ```
    pub fn list() -> Vec<(String, String)> {
        let files: Vec<String> = Asset::iter().map(|a| a.to_string()).collect();
        let mut result: Vec<(String, String)> = files
            .iter()
            .map(|x| {
                let file = Asset::get(x).unwrap();
                let header = &String::from_utf8_lossy(&file.data)[2..];
                (
                    x.to_string(),
                    header.split('\n').into_iter().next().unwrap().to_string(),
                )
            })
            .collect();
        result.sort_by(|(a, _), (b, _)| a.cmp(b));
        result
    }

    /// Map a String to a vector of words.
    /// ```
    /// use phonetic::{Phonetic, Alphabet};
    ///
    /// let phonetic = Phonetic::load(Alphabet::nato).unwrap();
    /// let words = phonetic.string_to_words("abc".to_string());
    /// assert_eq!(words, ["Alpha", "Bravo", "Charlie"]);
    /// ```
    pub fn string_to_words(&self, s: String) -> Vec<String> {
        // Vector we'll eventually return
        let mut words = Vec::new();

        // The algorithm works as follows (using "foobar" as an input):
        // - We start by creating an ngram the size of `self.max_ngram_len` ("foo")
        // - If we don't find a match in our alphabet, we decrease the size of our
        //   ngram ("fo") and try again
        // - If we do match, we add the result to our result vector and
        //   advance the start iterator to the character that wasn't part of the
        //   match.

        // We loop using an explicit iterator here, since we need to
        // advance the iterator manually
        let mut it = 0..s.len();

        // Start iterator
        while let Some(start) = it.next() {
            // Iterator counting down from `self.max_ngram_len` to 1, since
            // a the substring function that is used is excluding the end_index.
            // We start at `self.max_ngram_len`, since we want the largest match to
            // happen first (e.g. in Spanish, ll needs to match before l).
            for j in (1..=self.max_ngram_len).rev() {
                // Define the end index
                let end = start + j;

                // Make sure we don't go past the end of the string
                if end <= s.len() {
                    // Create an ngram
                    let ngram = s.substring(start, end).to_string();

                    // If we have a match, we add it to our result vector and
                    // advance the start iterator.
                    // Extra advancement is only necessary if the ngram was larger than
                    // one character. We take consume the nth element from the iterator
                    // where n is the length of the ngram - 2. The number comes from the
                    // fact that it.nth(0) is the next element and the element we want to
                    // make sure is consumed is the length - 1.
                    // E.g. if the ngram was of length 2, we've already consumed the first
                    // at the start of the iterator and we would only need to consume the next one,
                    // which is it.nth(0). If the ngram was of length 3, we again, already
                    // consumed the first element and we need to the next two one (0 and 1),
                    // hence nth(1) or nth(3-2)
                    if let Some(word) = self.words.get(&ngram) {
                        words.push(word.clone());
                        if ngram.len() > 1 {
                            it.nth(ngram.len() - 2);
                            // And we break the inner loop, because we need to reset the end
                            break;
                        }
                    };
                }
            }
        }
        words
    }
}

impl std::fmt::Display for Phonetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.words
                .iter()
                .map(|s| format!("{} {}", s.0.clone().to_uppercase(), s.1))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

/// Load a phonetic alphabet from a string
/// ```
/// use phonetic::Phonetic;
/// use std::str::FromStr;
///
/// let phonetic = Phonetic::from_str("nato");
///
/// assert_eq!(phonetic.is_ok(), true);
/// ```
impl std::str::FromStr for Phonetic {
    type Err = AlphabetNotFoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = Alphabet::from_str(s).unwrap();
        Phonetic::load(n)
    }
}
