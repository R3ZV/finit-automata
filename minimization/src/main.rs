mod fa;

use crate::fa::dfa::DFA;

fn main() {
    let dfa = DFA::from_path(String::from("input"));
    let minimized_dfa = dfa.minimize();
    dbg!(minimized_dfa);
}
