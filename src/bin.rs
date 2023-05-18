use clap::Parser;
use colored::*;
use std::io::stdin;
use std::str::FromStr;
use tabular::{Row, Table};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Alphabet to use
    #[clap(short, long, env="SALPH", default_value_t = String::from("nato"), validator = salph::SpellingAlphabet::validate)]
    alphabet: String,
    sentence: Vec<String>,

    /// List available alphabets
    #[clap(short, long)]
    list_alphabets: bool,

    /// Show the contents of an alphabet
    #[clap(short, long, validator = salph::SpellingAlphabet::validate)]
    show_alphabet: Option<String>,

    /// Disable colored output (word = green , number = yellow)
    #[clap(short, long)]
    disable_color: bool,

    /// Separator to use when printing
    #[clap(short = 'S', long, default_value = " ")]
    separator: String,
}

fn main() {
    let cli = Args::parse();

    // List available alphabets
    if cli.list_alphabets {
        list_alphabets();
        return;
    }

    // Show the contents of an alphabet
    if let Some(alphabet) = cli.show_alphabet {
        println!("{}", salph::SpellingAlphabet::from_str(&alphabet).unwrap());
        return;
    }

    // Select current alphabet
    let alphabet = salph::SpellingAlphabet::from_str(&cli.alphabet).unwrap();

    // Read the sentence from either stdin or arguments
    let sentence: Vec<String> = if cli.sentence.is_empty() {
        read_from_stdin()
    } else {
        cli.sentence.into_iter().collect()
    };

    // Create a table with every letter mapped to a word from the alphabet
    let mut table = Table::new("{:<}  {:<}");
    for word in sentence {
        let spellings = alphabet
            .string_to_spellings(&word)
            .iter()
            .map(|w| {
                if cli.disable_color {
                    w.to_string()
                } else if w.is_number {
                    w.spelling.yellow().to_string()
                } else {
                    w.spelling.green().to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(&cli.separator);
        let word = if cli.disable_color {
            word.clone()
        } else {
            word.bright_cyan().bold().to_string()
        };
        table.add_row(Row::new().with_cell(&word).with_cell(spellings));
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
    for alphabet in salph::SpellingAlphabet::list() {
        println!("  - {}: {}", alphabet.0, alphabet.1);
    }
}
