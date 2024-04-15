#ifndef NFA_H
#define NFA_H

#include "transition.h"

#include <iostream>
#include <fstream>
#include <vector>
#include <map>

class NFA {
    const int starting_node;
    const std::vector<std::vector<Transition>> adj;
    const std::vector<int> final_states;
    std::map<int, int> normal;
    const std::vector<int> nodes;

    NFA (
        int nfa_starting_node,
        std::vector<std::vector<Transition>> nfa_adj,
        std::vector<int> nfa_f_states,
        std::map<int, int> nfa_normal,
        std::vector<int> nfa_nodes
    );

    void dfs(int node, const std::string &word, int word_pos, bool &correct);

public:
    static NFA from_fstream(std::ifstream& fin);

    bool try_word(std::string word);
    friend std::ostream &operator<<(std::ostream &out, const NFA &nfa);
};
#endif // NFA_H
