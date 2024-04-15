#include "dfa.h"

DFA::DFA (
    int dfa_starting_node,
    std::vector<std::vector<Transition>> dfa_adj,
    std::vector<int> dfa_f_states,
    std::map<int, int> dfa_normal,
    std::vector<int> dfa_nodes
) :
    starting_node(dfa_starting_node),
    adj(dfa_adj),
    final_states(dfa_f_states),
    normal(dfa_normal),
    nodes(dfa_nodes)
{}

void DFA::dfs(int node, const std::string &word, int word_pos, bool &correct) {
    if (word_pos == (int)word.size()) {
        for (int f_sate : final_states) {
            if (node == f_sate) {
                correct = true;
            }
        }
        return;
    }
    for (Transition new_node : adj[normal[node]]) {
        if (new_node.letter == word[word_pos]) {
            dfs(new_node.to, word, word_pos + 1, correct);
        }
    }
}
DFA DFA::from_fstream(std::ifstream& fin) {
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
    return DFA(starting_node, adj, final_states, normal, nodes);
}

bool DFA::try_word(std::string word) {
    bool correct = false;
    if (word == "L") {
        for (int f_node : final_states) {
            if (f_node == starting_node) {
                correct = true;
            }
        }
    } else {
        dfs(starting_node, word, 0, correct);
    }
    return correct;
}

std::ostream &operator<<(std::ostream &out, const DFA &dfa) {
    out << "==== DFA Properties ====\n";
    out << "Starts from node: " << dfa.starting_node << "\n";
    out << "Has " << dfa.nodes.size() << " nodes: \n";
    for (int node : dfa.nodes) {
        out << node << " ";
    }
    out << "\n";
    out << "Has the following transitions \n";
    for (int i = 0; i < (int)dfa.adj.size(); ++i) {
        for (Transition transit : dfa.adj[i]) {
            out << "From node [" << dfa.nodes[i] << "] to " << transit.to << " with letter " << transit.letter << "\n";
        }
    }
    out << "Has " << dfa.final_states.size() <<  " final state nodes: \n";
    for (int node : dfa.final_states) {
        out << node << " ";
    }
    out << "\n";
    out << "========================\n";
    return out;
}
