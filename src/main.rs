use std::io;
use std::io::{Read, Write};
use crate::hangman::Hangman;

mod hangman;

fn main() {
    let mut buf = String::new();
    println!("How many guesses? ");
    io::stdin().read_line(&mut buf)
        .expect("error reading input for 'How many guesses?'");

    let n_guesses: usize = buf.trim().parse()
        .expect("couldn't parse number of guesses");

    buf = String::new();
    println!("How long will the word be? ");
    io::stdin().read_line(&mut buf)
        .expect("error reading input for 'How long will the word be?'");
    let n_size: usize = buf.trim().parse()
        .expect("couldn't parse word length");

    let mut hg = Hangman::new(&String::from("words_alpha.txt"),n_size, n_guesses);

    hg.guess('c');
}
