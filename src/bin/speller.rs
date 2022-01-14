extern crate spellcheck;
use spellcheck::Speller;
use std::fs;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        writeln!(std::io::stderr(), "Usage: speller <training-file> <word>").unwrap();
        writeln!(std::io::stderr(), "Example: {} training.txt tometo", args[0]).unwrap();
        std::process::exit(1);
    }

    let mut speller = Speller {
        letters: "abcdefghijklmnopqrstuvwxyz".to_string(),
        n_words: HashMap::new()
    };
    let contents = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");
    speller.train(&contents);
    if let Some(w) = speller.correct(&args[2]) {
        println!("{} -> {}", &args[2], w);
    } else {
        println!("No correction available!");
    }
}
