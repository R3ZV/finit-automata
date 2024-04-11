pub mod dfa;

#[cfg(test)]
mod tests {
    use crate::fa::dfa::{Transition, DFA};
    use std::collections::HashSet;

    #[test]
    fn mergeing_states() {
        let initial_dfa = DFA::new(
            6,
            String::from("ab"),
            [
                HashSet::from([Transition::new(1, 'a'), Transition::new(2, 'b')]),
                HashSet::from([Transition::new(0, 'a'), Transition::new(3, 'b')]),
                HashSet::from([Transition::new(5, 'b')]),
                HashSet::from([Transition::new(4, 'a'), Transition::new(5, 'b')]),
                HashSet::from([Transition::new(4, 'a'), Transition::new(5, 'b')]),
                HashSet::from([Transition::new(5, 'b'), Transition::new(5, 'a')]),
            ]
            .to_vec(),
            0,
            HashSet::from([2, 3, 4]),
        );

        let expected_dfa = DFA::new(
            3,
            String::from("ab"),
            [
                HashSet::from([Transition::new(0, 'a'), Transition::new(1, 'b')]),
                HashSet::from([Transition::new(1, 'a'), Transition::new(2, 'b')]),
                HashSet::from([Transition::new(2, 'a'), Transition::new(2, 'b')]),
            ]
            .to_vec(),
            0,
            HashSet::from([1]),
        );

        assert_eq!(initial_dfa.minimize(), expected_dfa);
    }
}
