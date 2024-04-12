use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Transition {
    to: u32,
    with: char,
}

impl Transition {
    pub fn new(to: u32, with: char) -> Self {
        Self { to, with }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LNfa {
    states: u32,
    adj: HashMap<u32, HashSet<Transition>>,
    initial_state: u32,
    final_state: u32,
}

impl LNfa {
    pub fn new(
        states: u32,
        adj: HashMap<u32, HashSet<Transition>>,
        initial_state: u32,
        final_state: u32,
    ) -> Self {
        Self {
            states,
            adj,
            initial_state,
            final_state,
        }
    }

    pub fn get_adj(&self) -> HashMap<u32, HashSet<Transition>> {
        self.adj.clone()
    }

    pub fn get_initial_state(&self) -> u32 {
        self.initial_state
    }

    pub fn get_final_state(&self) -> u32 {
        self.final_state
    }

    pub fn get_states(&self) -> u32 {
        self.states
    }
}
