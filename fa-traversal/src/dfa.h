#ifndef DFA_H
#define DFA_H

#include "transition.h"

#include <iostream>
#include <fstream>
#include <vector>
#include <map>

class DFA {
    const int starting_node;
    const std::vector<std::vector<Transition>> adj;
    const std::vector<int> final_states;
    std::map<int, int> normal;
    const std::vector<int> nodes;

    DFA (
        int dfa_starting_node,
        std::vector<std::vector<Transition>> dfa_adj,
        std::vector<int> dfa_f_states,
        std::map<int, int> dfa_normal,
        std::vector<int> dfa_nodes
    );

    void dfs(int node, const std::string &word, int word_pos, bool &correct);

public:
    static DFA from_fstream(std::ifstream& fin);

    bool try_word(std::string word);
    friend std::ostream &operator<<(std::ostream &out, const DFA &dfa);
};
#endif // DFA_H
