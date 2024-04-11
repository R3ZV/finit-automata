use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Transition {
    to: u32,
    with: char,
}

impl Transition {
    pub fn new(to: u32, with: char) -> Self {
        Transition { to, with }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DFA {
    states: u32,
    alphabet: String,
    adj: Vec<HashSet<Transition>>,
    initial_state: u32,
    final_states: HashSet<u32>,
}

impl DFA {
    pub fn new(
        states: u32,
        alphabet: String,
        adj: Vec<HashSet<Transition>>,
        initial_state: u32,
        final_states: HashSet<u32>,
    ) -> Self {
        DFA {
            states,
            alphabet,
            adj,
            initial_state,
            final_states,
        }
    }

    pub fn from_path(path: String) -> Self {
        let file_content = fs::read_to_string(path).expect("File not found");
        let mut input = file_content.lines();

        let states: u32 = input
            .next()
            .expect("Couldn't read line")
            .parse()
            .expect("Couldn't parse to u32 the number of states");

        let alphabet: String = input.next().expect("Couldn't read line").to_string();

        let num_transitions: u32 = input
            .next()
            .expect("Couldn't read line")
            .parse()
            .expect("Couldn't parse to u32 the number of transitions");

        let mut adj: Vec<HashSet<Transition>> = vec![HashSet::new(); num_transitions as usize];
        for _ in 0..num_transitions {
            let mut transition = input.next().expect("Couldn't read line").split(" ");
            let from: u32 = transition
                .next()
                .expect("Couldn't read the first element of the transition")
                .parse()
                .expect("Couldn't parse to u32 the first element of the transition (from)");

            let to: u32 = transition
                .next()
                .expect("Couldn't read the second element of the transition")
                .parse()
                .expect("Couldn't parse to u32 the second element of the transition (from)");

            let with: char = transition
                .next()
                .expect("Couldn't read the third element of the transition")
                .chars()
                .nth(0)
                .expect("The value for the third element of the transition doesn't contain 1 char");
            adj[from as usize].insert(Transition::new(to, with));
        }

        let initial_state: u32 = input
            .next()
            .expect("Couldn't read the value for initial state")
            .parse()
            .expect("Couldn't parse the initla state");

        let num_final_states: u32 = input
            .next()
            .expect("Couldn't read the value for  num_final_states")
            .parse()
            .expect("Couldn't parse the num_final_states");

        let mut final_states: HashSet<u32> = HashSet::with_capacity(num_final_states as usize);
        for f_state in input.next().expect("Couldn't read line").split(" ") {
            let f_state: u32 = f_state
                .parse()
                .expect("Couldn't parse to u32 a final state");
            final_states.insert(f_state);
        }

        DFA::new(states, alphabet, adj, initial_state, final_states)
    }

    fn is_final_state(&self, state: &u32) -> bool {
        self.final_states.contains(&state)
    }

    fn can_mark(
        &self,
        marked_states: &Vec<Vec<bool>>,
        state_1: &u32,
        state_2: &u32,
        with: &char,
    ) -> bool {
        let new_state_1: Option<u32> = self.adj[*state_1 as usize].iter().find_map(|transition| {
            if transition.with == *with {
                Some(transition.to)
            } else {
                None
            }
        });

        let new_state_2: Option<u32> = self.adj[*state_2 as usize].iter().find_map(|transition| {
            if transition.with == *with {
                Some(transition.to)
            } else {
                None
            }
        });

        if let Some(new_state_1) = new_state_1 {
            if let Some(new_state_2) = new_state_2 {
                if marked_states[new_state_1 as usize][new_state_2 as usize] {
                    return true;
                }
            }
        }
        false
    }

    pub fn minimize(&self) -> Self {
        let mut marked_states: Vec<Vec<bool>> =
            vec![vec![false; self.states as usize]; self.states as usize];

        for state_1 in 0..self.states {
            for state_2 in 0..state_1 {
                if self.is_final_state(&state_1) && !self.is_final_state(&state_2) {
                    marked_states[state_1 as usize][state_2 as usize] = true;
                }

                if !self.is_final_state(&state_1) && self.is_final_state(&state_2) {
                    marked_states[state_1 as usize][state_2 as usize] = true;
                }
            }
        }

        loop {
            let mut marked = false;
            for state_1 in 0..self.states {
                for state_2 in 0..state_1 {
                    if !marked_states[state_1 as usize][state_2 as usize] {
                        for chr in self.alphabet.chars() {
                            if self.can_mark(&marked_states, &state_1, &state_2, &chr) {
                                marked_states[state_1 as usize][state_2 as usize] = true;
                                marked = true;
                            }
                        }
                    }
                }
            }
            if !marked {
                break;
            }
        }

        let mut mergeable: Vec<HashSet<u32>> = Vec::new();
        for state_1 in 0..self.states {
            for state_2 in 0..state_1 {
                if !marked_states[state_1 as usize][state_2 as usize] {
                    let mut merged = false;
                    for mergeable_states in mergeable.iter_mut() {
                        if mergeable_states.contains(&state_1)
                            && !mergeable_states.contains(&state_2)
                        {
                            mergeable_states.insert(state_2);
                            merged = true;
                        } else if !mergeable_states.contains(&state_1)
                            && mergeable_states.contains(&state_2)
                        {
                            merged = true;
                            mergeable_states.insert(state_1);
                        } else if mergeable_states.contains(&state_1)
                            && mergeable_states.contains(&state_2)
                        {
                            merged = true;
                        }
                    }
                    if !merged {
                        mergeable.push(HashSet::from([state_1, state_2]));
                    }
                }
            }
        }
        for state in 0..self.states {
            if !mergeable.iter().any(|row| row.contains(&state)) {
                mergeable.push(HashSet::from([state]));
            }
        }

        return self.merge_states(mergeable);
    }

    fn merge_states(&self, mergeable_states: Vec<HashSet<u32>>) -> Self {
        let mut parent: Vec<u32> = vec![0; self.states as usize];
        for state in 0..self.states {
            let mut parent_pos = 0;
            for (pos, states) in mergeable_states.iter().enumerate() {
                if states.contains(&state) {
                    parent_pos = pos;
                }
            }
            parent[state as usize] = parent_pos as u32;
        }

        let mut new_adj: Vec<HashSet<Transition>> =
            vec![HashSet::new(); mergeable_states.len() as usize];
        for state in 0..self.states {
            for transition in self.adj[state as usize].iter() {
                let new_transition =
                    Transition::new(parent[transition.to as usize], transition.with);

                new_adj[parent[state as usize] as usize].insert(new_transition);
            }
        }

        let mut new_final_states: HashSet<u32> = HashSet::new();
        for final_state in self.final_states.iter() {
            new_final_states.insert(parent[*final_state as usize]);
        }

        Self::new(
            mergeable_states.len() as u32,
            self.alphabet.clone(),
            new_adj,
            parent[self.initial_state as usize],
            new_final_states,
        )
    }
}
