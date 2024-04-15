#include "dfa.h"
#include "nfa.h"
#include "lnfa.h"
#include <cstdlib>

void help() {
    std::cout << "Usage: \n";
    std::cout << "./cli [option]\n";
    std::cout << "Where option is one of: dfa, nfa, lnfa\n";
}

std::vector<std::string> read_words(std::ifstream &fin) {
    int word_cnt;
    fin >> word_cnt;
    std::vector<std::string> words(word_cnt);
    for (std::string &word : words) {
        fin >> word;
    }
    return words;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        help();
        std::exit(1);
    }
    std::string cli_option = argv[1];
    std::ifstream fin("input.txt");
    if (cli_option == "dfa") {
        DFA dfa = DFA::from_fstream(fin);
        std::vector<std::string> words = read_words(fin);
        for (std::string word : words) {
            if (dfa.try_word(word)) {
                std::cout << word << " has been accepted!\n";
            } else {
                std::cout << word << " has been rejected!\n";
            }
        }
    } else if (cli_option == "nfa") {
        NFA nfa = NFA::from_fstream(fin);
        std::vector<std::string> words = read_words(fin);
        for (std::string word : words) {
            if (nfa.try_word(word)) {
                std::cout << word << " has been accepted!\n";
            } else {
                std::cout << word << " has been rejected!\n";
            }
        }
    } else if (cli_option == "lnfa") {
        LNFA lnfa = LNFA::from_fstream(fin);
        std::vector<std::string> words = read_words(fin);
        for (std::string word : words) {
            if (lnfa.try_word(word)) {
                std::cout << word << " has been accepted!\n";
            } else {
                std::cout << word << " has been rejected!\n";
            }
        }
    } else {
        std::cout << "Invalid option\n";
    }
    return 0;
}
