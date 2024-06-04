mod dpda;

use dpda::DPDA;
use std::fs::read_to_string;

fn main() {
    let mut dpda = DPDA::from_file("automata".to_string());
    let words = read_to_string("words").expect("No file for words");
    for word in words.lines() {
        if dpda.try_word(word.to_string()) {
            println!("{} was accepted!", word);
        } else {
            println!("{} was rejected!", word);
        }
    }
}
