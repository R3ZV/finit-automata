use std::collections::{HashMap, HashSet};

use super::lnfa::{LNfa, Transition};

pub struct RegexProcessor {
    regex: String,
}

impl RegexProcessor {
    pub fn new(regex: String) -> Self {
        Self { regex }
    }

    fn symbol_expression(&self, symbol: &char, states: &mut u32) -> LNfa {
        *states += 2;

        let initial_state = *states - 2;
        let final_state = *states - 1;

        let mut adj = HashMap::new();
        adj.insert(
            initial_state,
            HashSet::from([Transition::new(final_state, *symbol)]),
        );
        return LNfa::new(2, adj, initial_state, final_state);
    }

    fn kleene_star(&self, curr_lnfa: LNfa, states: &mut u32) -> LNfa {
        *states += 2;

        let prev_initial_state = curr_lnfa.get_initial_state();
        let prev_final_state = curr_lnfa.get_final_state();
        let new_initial_state = *states - 2;
        let new_final_state = *states - 1;

        let mut new_adj = curr_lnfa.get_adj();
        new_adj.insert(
            new_initial_state,
            HashSet::from([
                Transition::new(prev_initial_state, 'L'),
                Transition::new(new_final_state, 'L'),
            ]),
        );
        new_adj.insert(
            prev_final_state,
            HashSet::from([
                Transition::new(new_final_state, 'L'),
                Transition::new(prev_initial_state, 'L'),
            ]),
        );

        let prev_states = curr_lnfa.get_states();
        return LNfa::new(prev_states + 2, new_adj, new_initial_state, new_final_state);
    }

    pub fn generate(&mut self) -> LNfa {
        // For '|' and '." we will always assume
        // that the next char is a '('
        let mut states = 0;
        let mut lnfas = Vec::new();
        for i in 0..self.regex.len() {
            let ch = self.regex.chars().nth(i).expect("No more chars");
            if ch.is_alphabetic() {
                lnfas.push(self.symbol_expression(&ch, &mut states));
            } else if ch == '*' {
                let curr_lnfa = lnfas.pop().unwrap();
                lnfas.push(self.kleene_star(curr_lnfa, &mut states));
            } else if ch == '(' {
                todo!();
            } else if ch == '|' {
                // a | (b | (c))
                todo!();
            } else if ch == '.' {
                todo!();
            }
        }
        lnfas.first().unwrap().clone()
    }
}
