use std::{env, io::stdin};

// cont ALPHABET = [""]
const ALPHABET: [&str; 26] = [
    "alfa", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india", "juliett",
    "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra", "tango",
    "uniform", "victor", "whiskey", "x-ray", "yankee", "zulu",
];

fn main() {
    let args: Vec<String> = env::args().collect();

    let sentence: Vec<String> = if args.len() > 1 {
        args.as_slice()[1..]
            .iter()
            .map(|x| x.to_ascii_lowercase())
            .collect()
    } else {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input
            .trim()
            .split(' ')
            .map(|s| s.to_string().to_ascii_lowercase())
            .collect()
    };

    for w in sentence {
        print!("{}: ", w);
        for c in w.chars() {
            print!("{} ", ALPHABET[(c as usize) - 97]);
        }
        println!();
    }
}
