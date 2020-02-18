use std::fs;
use std::collections::{BTreeMap, BTreeSet};

pub struct Hangman {
    word_length: usize,
    total_guesses: usize,
    letters_guessed: BTreeSet<char>,
    words: Vec<String>,
}

impl Hangman {
    pub fn new(dictionary_path: &String, word_length: usize, total_guesses: usize) -> Hangman {
        let mut words: Vec<String> = fs::read_to_string(dictionary_path)
            .expect("Failed to read dictionary!")
            .split("\n")
            .filter(|word| word.len()==word_length)
            .map(|word| String::from(word).trim().to_string())
            .collect();

        Hangman {
            word_length: word_length,
            total_guesses: total_guesses,
            letters_guessed: BTreeSet::new(),
            words: words
        }
    }

    pub fn guess(&mut self, guess: char) {
        if self.letters_guessed.contains(&guess) {
            return
        }
        let mut families = self.get_families(guess);
        self.letters_guessed.insert(guess);

        let mut biggest = families.get(families.keys().next().unwrap()).unwrap();
        for (k,v) in families.iter() {
           if v.len() > biggest.len() {
               biggest = v;
           }
        }
        self.words = biggest.clone();
        println!("{:?}",self.words);

    }

    fn get_families(&self, contains: char) -> BTreeMap<String,Vec<String>> {
        let mut contains_char = self.words
            .iter()
            .filter(|word| word.contains(contains));
        let mut families: BTreeMap<String,Vec<String>> = BTreeMap::new();
        for word in contains_char {
            let family = Hangman::family_key(&word, contains);
            if families.contains_key(&family) {
                families.get_mut(&family).unwrap().push(word.clone());
            } else {
               families.insert(family,vec![word.clone()]);
            }
        }
        families
    }

    fn family_key(word: &String, character: char) -> String {
        let mut out = String::new();
        for c in word.chars() {
            if c == character {
                out.push(c);
            } else {
                out.push('_');
            }
        }
        out
    }
}
