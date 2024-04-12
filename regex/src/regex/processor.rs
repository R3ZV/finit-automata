use std::collections::{HashMap, HashSet};

use super::lnfa::{LNfa, Transition};

pub struct RegexProcessor {
    regex: String,
    states: u32,
}

impl RegexProcessor {
    pub fn new(regex: String, states: u32) -> Self {
        Self { regex, states }
    }

    fn symbol_expression(&mut self, symbol: &char) -> LNfa {
        self.states += 2;

        let initial_state = self.states - 2;
        let final_state = self.states - 1;

        let mut adj = HashMap::new();
        adj.insert(
            initial_state,
            HashSet::from([Transition::new(final_state, *symbol)]),
        );
        return LNfa::new(2, adj, initial_state, final_state);
    }

    fn kleene_star(&mut self, curr_lnfa: LNfa) -> LNfa {
        self.states += 2;

        let prev_initial_state = curr_lnfa.get_initial_state();
        let prev_final_state = curr_lnfa.get_final_state();
        let new_initial_state = self.states - 2;
        let new_final_state = self.states - 1;

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

    fn concat(&self, first_lnfa: LNfa, second_lnfa: LNfa) -> LNfa {
        let mut new_adj = first_lnfa.get_adj();
        new_adj.extend(second_lnfa.get_adj());
        new_adj.insert(
            first_lnfa.get_final_state(),
            HashSet::from([Transition::new(second_lnfa.get_initial_state(), 'L')]),
        );
        LNfa::new(
            first_lnfa.get_states() + second_lnfa.get_states(),
            new_adj,
            first_lnfa.get_initial_state(),
            second_lnfa.get_final_state(),
        )
    }

    fn unite(&mut self, first_lnfa: LNfa, second_lnfa: LNfa) -> LNfa {
        let new_states = first_lnfa.get_states() + second_lnfa.get_states() + 2;
        let new_initial_state = new_states - 2;
        let new_final_state = new_states - 1;

        let mut new_adj = first_lnfa.get_adj();
        new_adj.extend(second_lnfa.get_adj());
        new_adj.insert(
            new_initial_state,
            HashSet::from([
                Transition::new(first_lnfa.get_initial_state(), 'L'),
                Transition::new(second_lnfa.get_initial_state(), 'L'),
            ]),
        );
        new_adj.insert(
            first_lnfa.get_final_state(),
            HashSet::from([Transition::new(new_final_state, 'L')]),
        );
        new_adj.insert(
            second_lnfa.get_final_state(),
            HashSet::from([Transition::new(new_final_state, 'L')]),
        );
        LNfa::new(new_states, new_adj, new_initial_state, new_final_state)
    }

    // start is the first position after the first '('
    // and returns the possition of the matching ')'
    fn parse_paranth(&self, start: usize) -> usize {
        let mut depth = 1;
        let mut matching_pos = 0;
        for i in start..self.regex.len() {
            let curr_char = self.regex.chars().nth(i).expect("No more chars");
            if curr_char == '(' {
                depth += 1;
            } else if curr_char == ')' {
                depth -= 1;
            }
            if depth == 0 {
                matching_pos = i;
                break;
            }
        }
        return matching_pos;
    }

    fn parse_sub_regex(&self, start: usize, end: usize) -> String {
        let mut sub_regex = String::new();
        for pos in start..end {
            let curr_char = self.regex.chars().nth(pos).expect("No more chars");
            sub_regex.push(curr_char);
        }
        return sub_regex;
    }

    pub fn generate(&mut self) -> LNfa {
        let mut lnfas = Vec::new();
        let mut i = 0;
        while i < self.regex.len() {
            let ch = self.regex.chars().nth(i).expect("No more chars");
            if ch.is_alphabetic() {
                lnfas.push(self.symbol_expression(&ch));
            } else if ch == '*' {
                let curr_lnfa = lnfas.pop().expect("No lnfas");
                lnfas.push(self.kleene_star(curr_lnfa));
            } else if ch == '|' {
                let matching_par_pos = self.parse_paranth(i + 2);
                let sub_regex = self.parse_sub_regex(i + 2, matching_par_pos);
                i = matching_par_pos;

                let right_lnfa = RegexProcessor::new(sub_regex, self.states).generate();
                let curr_lnfa = lnfas.pop().expect("No lnfas");
                lnfas.push(self.unite(curr_lnfa, right_lnfa));
            } else if ch == '.' {
                let matching_par_pos = self.parse_paranth(i + 2);
                let sub_regex = self.parse_sub_regex(i + 2, matching_par_pos);
                i = matching_par_pos;

                let right_lnfa = RegexProcessor::new(sub_regex, self.states).generate();
                let curr_lnfa = lnfas.pop().expect("No lnfas in concat");
                lnfas.push(self.concat(curr_lnfa, right_lnfa));
            }
            i += 1;
        }
        lnfas.first().expect("No lnfas").clone()
    }
}
