use std::fs::read_to_string;

#[derive(Clone, Debug)]
pub struct Transition {
    to: i32,
    with: char,
    rmv: char,
    add: String,
}

#[derive(Debug)]
pub struct DPDA {
    starting_state: i32,
    transitions: Vec<Vec<Transition>>,
    final_states: Vec<i32>,
}

impl Transition {
    pub fn new(to: i32, with: char, rmv: char, add: String) -> Self {
        Self { to, with, rmv, add }
    }
}

impl DPDA {
    pub fn from_file(path: String) -> Self {
        let automata = read_to_string(path).expect("No path was found to read the automata");
        let mut data = automata.lines();

        let starting_state: i32 = data
            .next()
            .expect("No starting node")
            .parse()
            .expect("Starting node is not a number");

        let nodes_nr: usize = data
            .next()
            .expect("No number of nodes")
            .parse()
            .expect("Number of nodes is not a number");

        let transitions_nr: i32 = data
            .next()
            .expect("No number of transitions")
            .parse()
            .expect("Number of transitions is not a number");

        let mut transitions: Vec<Vec<Transition>> = vec![Vec::new(); nodes_nr];
        for _ in 0..transitions_nr {
            let transition: Vec<&str> = data.next().unwrap().split(" ").collect();
            let from: usize = transition[0].parse().expect("From NAN");
            let to: i32 = transition[1].parse().expect("To NAN");
            let with: char = transition[2].chars().next().unwrap();
            let rmv: char = transition[3].chars().next().unwrap();
            let add: String = transition[4].to_string();

            transitions[from].push(Transition::new(to, with, rmv, add));
        }

        let final_states_nr = data
            .next()
            .expect("No number of final states")
            .parse()
            .expect("Number of final states NAN");

        let mut final_states: Vec<i32> = Vec::new();
        for _ in 0..final_states_nr {
            let f_node = data
                .next()
                .unwrap()
                .parse()
                .expect("One of the final states NAN");
            final_states.push(f_node);
        }
        Self {
            starting_state,
            transitions,
            final_states,
        }
    }

    fn step(&mut self, chr: char, node: &mut i32, stack: &mut Vec<char>) -> bool {
        let transition_to_take = self.transitions[*node as usize]
            .iter()
            .find(|t| t.with == chr);

        // no transition for our curr char to process
        if transition_to_take.is_none() {
            return false;
        }
        let transition_to_take = transition_to_take.unwrap();

        // empty stack
        if stack.last().is_none() {
            return false;
        }

        // stack doesn't have the char we need to remove
        if *stack.last().unwrap() != transition_to_take.rmv {
            return false;
        } else {
            stack.pop();
            for to_add in transition_to_take.add.chars() {
                stack.push(to_add);
            }
        }
        *node = transition_to_take.to;
        true
    }
    pub fn try_word(&mut self, word: String) -> bool {
        let mut curr_node = self.starting_state;
        let mut stack: Vec<char> = Vec::from(['$']);

        for chr in word.chars() {
            if self.step(chr, &mut curr_node, &mut stack) == false {
                return false;
            }
        }

        // accept only by final state
        return self.final_states.contains(&curr_node);
    }
}
