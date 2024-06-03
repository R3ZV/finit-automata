use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub struct Grammer {
    /// We use a HashMap because we want to index
    /// using non_terminal symbols
    ///
    /// We also use a HashSet to remove
    /// any redundance in the transitions
    /// i.e. A -> BC | BC should only be A -> BC
    rules: HashMap<String, HashSet<String>>,

    /// The word to check if it can be obtained by the grammer
    word: String,
}

impl Grammer {
    /// The grammer has to be in Chmosky form i.e.:
    /// A -> BC         at most 2 non terminals
    /// A -> a          terminal symbol
    /// A -> lambda (denoted as     null string
    pub fn from_file(path: String) -> Self {
        let content = fs::read_to_string(path).expect("Path file not found");
        let mut lines = content.lines();

        let word = lines.next().expect("No word found").to_string();
        let mut non_terminals: HashMap<String, HashSet<String>> = HashMap::new();
        for line in lines.into_iter() {
            let mut tokens = line.split(" ");
            let source = tokens.next().expect("Production has no source");
            for token in tokens {
                if token == "->" || token == "|" {
                    continue;
                }

                non_terminals
                    .entry(source.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(token.to_string());
            }
        }
        Self {
            rules: non_terminals,
            word,
        }
    }

    pub fn try_word(&self) -> bool {
        let mut cyk_table: Vec<Vec<HashSet<String>>> =
            vec![vec![HashSet::new(); self.word.len()]; self.word.len()];

        self.cyk(&mut cyk_table);

        cyk_table[self.word.len() - 1][0].contains("S")
    }

    fn cyk(&self, table: &mut Vec<Vec<HashSet<String>>>) {
        for (i, terminal) in self.word.chars().enumerate() {
            for production in &self.rules {
                if production.1.contains(&terminal.to_string()) {
                    table[0][i].insert(production.0.to_string());
                }
            }
        }
        for length in 1..self.word.len() {
            for start_idx in 0..self.word.len() - length {
                for partition in 0..length {
                    for production in &self.rules {
                        for non_terminals in production.1 {
                            if non_terminals.len() == 1 {
                                continue;
                            }
                            let first_non_terminal = non_terminals.chars().nth(0).unwrap();
                            let second_non_terminal = non_terminals.chars().nth(1).unwrap();
                            if table[partition][start_idx].contains(&first_non_terminal.to_string())
                                && table[length - partition - 1][start_idx + partition + 1]
                                    .contains(&second_non_terminal.to_string())
                            {
                                table[length][start_idx].insert(production.0.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}
