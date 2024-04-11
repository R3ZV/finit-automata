use std::collections::HashSet;

use super::lnfa::{LNfa, Transition};

pub struct RegexProcessor {
    regex: String,
    alphabet: String,
}

impl RegexProcessor {
    pub fn new(regex: String) -> Self {
        let mut alphabet = String::new();
        for ch in regex.chars() {
            if ch.is_alphabetic() {
                alphabet.push(ch);
            }
        }
        Self { regex, alphabet }
    }

    pub fn generate(&self) -> LNfa {
        // TODO: change this to return actual LNFA
        LNfa::new(
            3,
            [HashSet::from([Transition::new(2, 'a')])].to_vec(),
            0,
            10,
        )
    }
}
