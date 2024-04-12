pub mod lnfa;
pub mod processor;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::{
        lnfa::{LNfa, Transition},
        processor::RegexProcessor,
    };

    #[test]
    fn test_lambda_symbol() {
        let generated_lnfa = RegexProcessor::new(String::from("L"), 0).generate();
        let expected_lnfa = LNfa::new(
            2,
            HashMap::from([(0, HashSet::from([Transition::new(1, 'L')]))]),
            0,
            1,
        );
        assert_eq!(generated_lnfa, expected_lnfa);
    }

    #[test]
    fn test_symbol() {
        let generated_lnfa = RegexProcessor::new(String::from("a"), 0).generate();
        let expected_lnfa = LNfa::new(
            2,
            HashMap::from([(0, HashSet::from([Transition::new(1, 'a')]))]),
            0,
            1,
        );
        assert_eq!(generated_lnfa, expected_lnfa);
    }
}
