#![doc(html_root_url = "https://docs.rs/salph/1/")]
//! salph is a library that allows you to transform [`&str`]s to [`Vec`]s of words
//! from spelling alphabets.
//!
//! Usage:
//! ```
//! use salph::{SpellingAlphabet, Alphabet, Spelling};
//! use std::str::FromStr;
//!
//! // Load a spelling alphabet using the Alphabet enum
//! let spelling_alphabet = SpellingAlphabet::load(Alphabet::nato).unwrap();
//! let word_list = spelling_alphabet.str_to_spellings("abc123");
//! assert_eq!(word_list, [
//!     Spelling { spelling: "Alpha".to_string(), is_number: false },
//!     Spelling { spelling: "Bravo".to_string(), is_number: false },
//!     Spelling { spelling: "Charlie".to_string(), is_number: false },
//!     Spelling { spelling: "one".to_string(), is_number: true },
//!     Spelling { spelling: "two".to_string(), is_number: true },
//!     Spelling { spelling: "three".to_string(), is_number: true },
//! ]);
//!
//! // Load a spelling alphabet using an &str
//! let spelling_alphabet = SpellingAlphabet::from_str("nato").unwrap();
//! let word_list = spelling_alphabet.str_to_spellings("abc")
//!         .iter()
//!         .map(|x| x.spelling.clone())
//!         .collect::<Vec<_>>();
//! assert_eq!(word_list, ["Alpha", "Bravo", "Charlie"]);
//! ```
//!
//! Supported alphabets can be found in the [`Alphabet`] struct

include!(concat!(env!("OUT_DIR"), "/alphabet_kinds.rs"));

use core::fmt;
use indexmap::IndexMap;
use rust_embed::RustEmbed;
use std::{cmp::Reverse, str::FromStr};
use substring::Substring;

#[derive(RustEmbed)]
#[folder = "alphabets"]
struct Asset;

// Struct representing an alphabet
#[derive(Debug, Clone)]
pub struct SpellingAlphabet {
    words: IndexMap<String, String>,
    max_ngram_len: usize,
}

// Error returned when an alphabet can't be found
#[derive(Debug)]
pub struct AlphabetNotFoundError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Spelling {
    pub spelling: String,
    pub is_number: bool,
}

impl fmt::Display for Spelling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.spelling)
    }
}

/// Struct that represents an Alphabet
impl SpellingAlphabet {
    /// Load an alphabet based on it's name
    /// ```
    /// use salph::{SpellingAlphabet, Alphabet};
    ///
    /// let spelling_alphabet = SpellingAlphabet::load(Alphabet::nato);
    ///
    /// assert_eq!(spelling_alphabet.is_ok(), true);
    /// ```
    pub fn load(name: Alphabet) -> Result<SpellingAlphabet, AlphabetNotFoundError> {
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

        Ok(SpellingAlphabet {
            words,
            max_ngram_len,
        })
    }

    /// Validate if there's a mapping for the given alphabet
    /// ```
    /// use salph::SpellingAlphabet;
    ///
    /// let res = SpellingAlphabet::validate("nato");
    /// assert_eq!(res.is_ok(), true);
    ///
    /// let res = SpellingAlphabet::validate("nonexistent");
    /// assert_eq!(res.is_err(), true);
    ///
    /// ```
    pub fn validate(s: &str) -> Result<String, String> {
        match Alphabet::from_str(s) {
            Ok(_) => Ok(s.to_string()),
            Err(_) => Err(format!("Unknown alphabet: {}", s)),
        }
    }

    /// List all available alphabets. This function returns a [`Vec`] of tuples
    /// containing the `(alphabet abbreviation, long name)` (e.g. `("fr-BE", "French (Belgium)")`)
    /// ```
    /// use salph::SpellingAlphabet;
    ///
    /// let alphabets = SpellingAlphabet::list();
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
                    header.split('\n').next().unwrap().to_string(),
                )
            })
            .collect();
        result.sort_by(|(a, _), (b, _)| a.cmp(b));
        result
    }

    /// Map a String to a vector of `Spelling`s.
    /// ```
    /// use salph::{SpellingAlphabet, Alphabet};
    ///
    /// let spelling_alphabet = SpellingAlphabet::load(Alphabet::nato).unwrap();
    /// let words = spelling_alphabet
    ///         .str_to_spellings("Abc98")
    ///         .iter()
    ///         .map(|x| x.spelling.clone())
    ///         .collect::<Vec<_>>();
    /// assert_eq!(words, ["Alpha", "Bravo", "Charlie", "nine", "eight"]);
    /// ```
    pub fn str_to_spellings(&self, s: &str) -> Vec<Spelling> {
        // Vector we'll eventually return
        let mut spellings = Vec::new();

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
                    let ngram = s.substring(start, end).to_string().to_lowercase();

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
                        spellings.push(Spelling {
                            spelling: word.clone(),
                            is_number: ngram.parse::<i32>().is_ok(),
                        });
                        if ngram.len() > 1 {
                            it.nth(ngram.len() - 2);
                            // And we break the inner loop, because we need to reset the end
                            break;
                        }
                    };
                }
            }
        }
        spellings
    }
}

impl std::fmt::Display for SpellingAlphabet {
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

/// Load a spelling alphabet from a string
/// ```
/// use salph::SpellingAlphabet;
/// use std::str::FromStr;
///
/// let spelling_alphabet = SpellingAlphabet::from_str("nato");
///
/// assert_eq!(spelling_alphabet.is_ok(), true);
/// ```
impl std::str::FromStr for SpellingAlphabet {
    type Err = AlphabetNotFoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = Alphabet::from_str(s).unwrap();
        SpellingAlphabet::load(n)
    }
}
