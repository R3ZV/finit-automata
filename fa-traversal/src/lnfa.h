#ifndef LNFA_H
#define LNFA_H

#include "transition.h"

#include <iostream>
#include <fstream>
#include <vector>
#include <map>

class LNFA {
    const int starting_node;
    const std::vector<std::vector<Transition>> adj;
    const std::vector<int> final_states;
    std::map<int, int> normal;
    const std::vector<int> nodes;
    std::map<int, int> states;

    LNFA (
        int lnfa_starting_node,
        std::vector<std::vector<Transition>> lnfa_adj,
        std::vector<int> lnfa_f_states,
        std::map<int, int> lnfa_normal,
        std::vector<int> lnfa_nodes,
        std::map<int, int> states
    );

    void dfs(int node, const std::string &word, int word_pos, bool &correct);

public:
    static LNFA from_fstream(std::ifstream& fin);

    bool try_word(std::string word);
    friend std::ostream &operator<<(std::ostream &out, const LNFA &lnfa);
};
#endif // LNFA_H
