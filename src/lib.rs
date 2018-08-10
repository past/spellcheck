//! A spell-checker based on the statistical algorithm described by Peter Norvig
//! in http://norvig.com/spell-correct.html
//!
//! This is a port of the JavaScript implementation of the algorithm from
//! https://github.com/past/speller
//!
//! Usage requires a two-step process:
//! 1) call speller.train() one or more times with a large text to train the language model
//! 2) call speller.correct(word) to retrieve the correction for the specified word

extern crate regex;
use regex::Regex;
use std::collections::HashMap;

pub struct Speller {
    /// The letters of the alphabet.
    pub letters: String,
    /// A frequency map of words to the number of times they were found during
    /// training.
    pub n_words: HashMap<String, u32>
}

impl Speller {
    /// A function that trains the language model with the words in the supplied text.
    /// Multiple invocation of this function can extend the training of the model.
    pub fn train(&mut self, text: &str) {
        let re = Regex::new(r"[a-z]+").unwrap();
        let lc_text = text.to_lowercase();
        for m in re.find_iter(&lc_text) {
            let count = self.n_words.entry(m.as_str().to_string()).or_insert(0);
            *count += 1;
        }
    }

    /// A function that returns the correction for the specified word.
    pub fn correct(&mut self, word: &str) -> String {
        // A word in our word frequency map is already correct.
        if self.n_words.contains_key(word) {
            return word.to_string();
        }

        let mut candidates: HashMap<u32, String> = HashMap::new();
        let list = self.edits(word);

        // Try to find candidate corrections in the edits of the word.
        for edit in &list {
            if let Some(value) = self.n_words.get(edit) {
                candidates.insert(*value, edit.to_string());
            }
        }
        if let Some(c) = candidates.iter().max_by_key(|&entry| entry.0) {
            return c.1.to_string();
        }

        // Try to find candidate corrections in the edits of the edits.
        for edit in &list {
            for w in self.edits(&edit) {
                if let Some(value) = self.n_words.get(&w) {
                    candidates.insert(*value, w);
                }
            }
        }
        if let Some(c) = candidates.iter().max_by_key(|&entry| entry.0) {
            return c.1.to_string();
        }

        // Can't find a correction, return the input unchanged.
        word.to_string()
    }

    /// A function that returns the set of possible corrections of the specified word.
    /// The edits can be deletions, insertions, alterations or transpositions.
    fn edits(&mut self, word: &str) -> Vec<String> {
        let mut results = Vec::new();
        // deletion
        for i in 0 .. word.len() {
            let (first, last) = word.split_at(i);
            results.push([first, &last[1..]].concat());
        }
        // transposition
        for i in 0 .. word.len() - 1 {
            let (first, last) = word.split_at(i);
            results.push([first, &last[1..2], &last[..1], &last[2..]].concat());
        }
        // alteration
        for i in 0 .. word.len() {
            for c in self.letters.chars() {
                let (first, last) = word.split_at(i);
                let mut buffer = [0; 1];
                let result = c.encode_utf8(&mut buffer);
                results.push([first, result, &last[1..]].concat());
            }
        }
        // insertion
        for i in 0 .. word.len() + 1 {
            for c in self.letters.chars() {
                let (first, last) = word.split_at(i);
                let mut buffer = [0; 1];
                let result = c.encode_utf8(&mut buffer);
                results.push([first, result, last].concat());
            }
        }

        results
    }
}
