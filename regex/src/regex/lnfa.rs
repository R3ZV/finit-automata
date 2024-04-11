use std::collections::HashSet;

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

#[derive(Debug)]
pub struct LNfa {
    states: u32,
    adj: Vec<HashSet<Transition>>,
    initial_state: u32,
    final_state: u32,
}

impl LNfa {
    pub fn new(
        states: u32,
        adj: Vec<HashSet<Transition>>,
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
}
