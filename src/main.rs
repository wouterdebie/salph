use clap::Parser;
use indexmap::IndexMap;
use rust_embed::RustEmbed;
use std::{cmp::Reverse, fmt::Display, io::stdin};
use substring::Substring;
use tabular::{Row, Table};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Alphabet to use
    #[clap(short, long, default_value_t = String::from("nato"), validator = validate_alphabet)]
    alphabet: String,
    sentence: Vec<String>,

    /// List available alphabets
    #[clap(short, long)]
    list_alphabets: bool,

    /// Show the contents of an alphabet
    #[clap(short, long, validator = validate_alphabet)]
    show_alphabet: Option<String>,
}
/// Validate if there's a mapping for the given alphabet
fn validate_alphabet(s: &str) -> Result<(), String> {
    Asset::iter()
        .any(|x| x == s)
        .then(|| ())
        .ok_or(format!("Unknown alphabet: {}", s))
}

#[derive(RustEmbed)]
#[folder = "alphabets"]
struct Asset;

fn main() {
    let cli = Args::parse();

    // List available alphabets
    if cli.list_alphabets {
        list_alphabets();
        return;
    }

    // Show the contents of an alphabet
    if let Some(alphabet) = cli.show_alphabet {
        println!("{}", Alphabet::load(alphabet));
        return;
    }

    // Select current alphabet
    let alphabet = Alphabet::load(cli.alphabet);

    // Read the sentence from either stdin or arguments
    let sentence: Vec<String> = if cli.sentence.is_empty() {
        read_from_stdin()
    } else {
        cli.sentence.into_iter().collect()
    };

    // Create a table with every letter mapped to a word from the alphabet
    let mut table = Table::new("{:<}  {:<}");
    for word in sentence {
        table.add_row(
            Row::new()
                .with_cell(&word)
                .with_cell(alphabet.string_to_words(word).join(" ")),
        );
    }
    print!("{}", table);
}

/// Read a sentence from stdin and convert it to a Vector of Strings
fn read_from_stdin() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().split(' ').map(|s| s.to_string()).collect()
}

/// List all available alphabets
fn list_alphabets() {
    println!("Available alphabets: ");
    for file in Asset::iter() {
        println!("  - {}", file.as_ref());
    }
}

// Struct representing an alphabet
struct Alphabet {
    words: IndexMap<String, String>,
    max_ngram_len: usize,
}

impl Alphabet {
    /// Load an alphabet based on it's name
    fn load(name: String) -> Alphabet {
        // Load the alphabet from an embedded asset into a utf8 string
        let alphabet_string = String::from_utf8_lossy(&Asset::get(&name).unwrap().data).to_string();

        // Split the string, filter out empty lines and turn it into a HashMap<String, String>
        let words: IndexMap<String, String> = alphabet_string
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| {
                let n: Vec<String> = x.splitn(2, ' ').map(|x| x.to_string()).collect();
                (n[0].to_lowercase(), n[1].clone())
            })
            .collect();

        let mut prefixes: Vec<_> = words.keys().collect();
        prefixes.sort_by_key(|b| Reverse(b.len()));
        let max_ngram_len = prefixes[0].len();

        Alphabet {
            words,
            max_ngram_len,
        }
    }

    /// Map a string to a vector of words.
    fn string_to_words(&self, s: String) -> Vec<String> {
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

impl Display for Alphabet {
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
