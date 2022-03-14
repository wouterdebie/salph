use clap::Parser;
use rust_embed::RustEmbed;
use std::{fmt::Display, io::stdin};
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
    // TODO: I don't like that the actual mapping happens inside the loop
    //       where formatting happens, but it's probably the most efficient way
    let mut table = Table::new("{:<}  {:<}");
    for word in sentence {
        table.add_row(
            Row::new().with_cell(&word).with_cell(
                word.to_lowercase()
                    .chars()
                    .map(|c| alphabet.char_to_word(c))
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
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
    words: Vec<String>,
}

impl Alphabet {
    /// Load an alphabet based on it's name
    fn load(name: String) -> Alphabet {
        // Load the alphabet from an embedded asset into a utf8 string
        let alphabet_string = String::from_utf8_lossy(&Asset::get(&name).unwrap().data).to_string();

        // Split the string and filter out empty lines
        let words: Vec<String> = alphabet_string
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();
        Alphabet { words }
    }

    /// Map a character to a word.
    /// TODO: this assumes that the character is lower case, since it uses the ASCII code.
    /// It might be nicer to map strings to strings probably to circumvent utf8 issues
    fn char_to_word(&self, c: char) -> String {
        self.words.get(c as usize - 97).unwrap().clone()
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.join("\n"))
    }
}
