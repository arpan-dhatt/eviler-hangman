use std::fs;
use std::collections::BTreeMap;

pub struct Hangman {
    word_length: usize,
    total_guesses: usize,
    letters_guessed: Vec<char>,
    words: Vec<String>,
}

impl Hangman {
    pub fn new(dictionary_path: &String, word_length: usize, total_guesses: usize) -> Hangman {
        let mut words: Vec<String> = fs::read_to_string(dictionary_path)
            .expect("Failed to read dictionary!")
            .split("\n")
            .filter(|word| word.len()==word_length)
            .map(|word| String::from(word))
            .collect();

        Hangman {
            word_length: word_length,
            total_guesses: total_guesses,
            letters_guessed: vec![],
            words: words
        }
    }

    pub fn guess(&mut self, guess: char) {

    }

    fn get_families(&self, words: Vec<String>, contains: char) {
        let mut contains_char = words
            .iter()
            .filter(|word| word.contains(contains));
        let families: BTreeMap<String,Vec<String>> = BTreeMap::new();
        for word in contains_char {

        }
    }

    fn family_key(word: &String, character: char) -> String {

    }
}
