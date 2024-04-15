#include "lnfa.h"

LNFA::LNFA (
    int nfa_starting_node,
    std::vector<std::vector<Transition>> nfa_adj,
    std::vector<int> nfa_f_states,
    std::map<int, int> nfa_normal,
    std::vector<int> nfa_nodes,
    std::map<int, int> states
) :
    starting_node(nfa_starting_node),
    adj(nfa_adj),
    final_states(nfa_f_states),
    normal(nfa_normal),
    nodes(nfa_nodes),
    states(states)
{}

void LNFA::dfs(int node, const std::string &word, int word_pos, bool &correct) {
    if (states.find(node) != states.end() && states[node] == word_pos) {
        return;
    }
    states[node] = word_pos;
    if (word_pos == (int)word.size()) {
        for (int f_sate : final_states) {
            if (node == f_sate) {
                correct = true;
            }
        }
    }
    for (Transition new_node : adj[normal[node]]) {
        if (new_node.letter == word[word_pos]) {
            dfs(new_node.to, word, word_pos + 1, correct);
        } else if (new_node.letter == 'L') {
            dfs(new_node.to, word, word_pos, correct);
        }
    }
}

LNFA LNFA::from_fstream(std::ifstream& fin) {
    int nodes_cnt;
    fin >> nodes_cnt;

    std::vector<int> nodes(nodes_cnt);
    std::map<int, int> normal;
    for (int i = 0; i < nodes_cnt; ++i) {
        fin >> nodes[i];
        normal[nodes[i]] = i;
    }

    int transitions_cnt;
    fin >> transitions_cnt;
    std::vector<std::vector<Transition>> adj(nodes_cnt);
    for (int i = 0; i < transitions_cnt; ++i) {
        int from, to;
        char cost;
        fin >> from >> to >> cost;
        adj[normal[from]].push_back(Transition(to, cost));
    }
    int starting_node;
    fin >> starting_node;

    int final_states_cnt;
    fin >> final_states_cnt;

    std::vector<int> final_states;
    for (int i = 0; i < final_states_cnt; ++i) {
        int f_state;
        fin >> f_state;
        final_states.push_back(f_state);
    }
    return LNFA(starting_node, adj, final_states, normal, nodes, {});
}

bool LNFA::try_word(std::string word) {
    bool correct = false;
    states = {};
    if (word == "L") {
        for (int f_node : final_states) {
            if (f_node == starting_node) {
                correct = true;
            }
        }
    }
    if (!correct) {
        dfs(starting_node, word, 0, correct);
    }
    return correct;
}

std::ostream &operator<<(std::ostream &out, const LNFA &nfa) {
    out << "==== LNFA Properties ====\n";
    out << "Starts from node: " << nfa.starting_node << "\n";
    out << "Has " << nfa.nodes.size() << " nodes: \n";
    for (int node : nfa.nodes) {
        out << node << " ";
    }
    out << "\n";
    out << "Has the following transitions \n";
    for (int i = 0; i < (int)nfa.adj.size(); ++i) {
        for (Transition transit : nfa.adj[i]) {
            out << "From node [" << nfa.nodes[i] << "] to " << transit.to << " with letter " << transit.letter << "\n";
        }
    }
    out << "Has " << nfa.final_states.size() <<  " final state nodes: \n";
    for (int node : nfa.final_states) {
        out << node << " ";
    }
    out << "\n";
    out << "========================\n";
    return out;
}
