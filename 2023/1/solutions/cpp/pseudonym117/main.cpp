#include <fstream>
#include <map>
#include <iostream>
#include <optional>
#include <string>

enum TrieMatchType {
    none,
    partial,
    full,
};

struct TrieMatch {
    const TrieMatchType type;
    const std::optional<int> value;

    TrieMatch() : type(TrieMatchType::none) {}
    TrieMatch(const int value) : type(TrieMatchType::full), value(value) {}

    const static TrieMatch partial;
private:
    TrieMatch(const TrieMatchType type) : type(type) {}
};
const TrieMatch TrieMatch::partial(TrieMatchType::partial);

class TrieNode {
    int value = std::numeric_limits<int>::min();
    std::map<char, TrieNode> children;
public:
    operator bool() const { return value != std::numeric_limits<int>::min(); }
    const TrieMatch at(const std::string_view& key) const {
        if (key.empty()) {
            if (*this) {
                return value;
            }
            if (!children.empty()) {
                return TrieMatch::partial;
            }
            return TrieMatch();
        }

        char c = key.front();
        if (children.contains(c)) {
            auto& child = children.at(c);
            auto next_key = key.substr(1);
            return child.at(next_key);
        }
        return TrieMatch();
    }
    
    int& operator[](const std::string_view& key) {
        if (key.empty()) {
            return value;
        }

        char c = key.front();
        auto& child = children[c];
        auto next_key = key.substr(1);
        return child[next_key];
    }
};

class Trie {
    TrieNode root;
public:
    Trie() {}
    Trie(const std::initializer_list<std::pair<std::string, int>>& initial) {
        add_all(initial);
    }

    void add_all(const std::initializer_list<std::pair<std::string, int>>& list) {
        auto& self = *this;
        for (const auto& [key, value] : list) {
            self[key] = value;
        }
    }

    const TrieMatch at(const std::string_view& key) const {
        return root.at(key);
    }

    int& operator[](const std::string_view& key) {
        return root[key];
    }
};



int main(int argc, char* argv[]) {
    if (argc < 2) {
        return -1;
    }

    char* filename = argv[1];
    std::ifstream file(filename);

    Trie lookup {
        {"0", 0},
        {"1", 1},
        {"2", 2},
        {"3", 3},
        {"4", 4},
        {"5", 5},
        {"6", 6},
        {"7", 7},
        {"8", 8},
        {"9", 9},
        {"zero", 0},
        {"one", 1},
        {"two", 2},
        {"three", 3},
        {"four", 4},
        {"five", 5},
        {"six", 6},
        {"seven", 7},
        {"eight", 8},
        {"nine", 9},
    };
    

    int sumOne = 0;
    int sumTwo = 0;
    std::string line;
    while (std::getline(file, line)) {
        int lineFirst = 0;
        int lineLast = 0;
        
        // technically can refactor this to use the lookup table instead... but i dont want to this late at night
        for (const auto working : line) {
            if (working >= '0' && working <= '9') {
                if (!lineFirst) {
                    lineFirst = working;
                }
                lineLast = working;
            }
        }
        if (lineFirst) {
            sumOne += (lineFirst - '0') * 10 + (lineLast - '0');
        }

        lineFirst = lineLast = 0;
        for (auto i = 0; i < line.size(); ++i) {
            std::optional<int> value;
            for (auto scanSize = 1; scanSize < line.size(); ++scanSize) {
                std::string_view current(line.c_str() + i, scanSize);

                auto match = lookup.at(current);
                
                if (match.type == TrieMatchType::full) {
                    value = match.value;
                    break;
                }
                if (match.type == TrieMatchType::partial) {
                    // can be more efficient if we save the current TrieNode state in the partial match and resume search from there,
                    // but i really dont think it will matter here.
                    continue;
                }
                if (match.type == TrieMatchType::none) {
                    break;
                }
            }
            
            if (value.has_value()) {
                if (!lineFirst) {
                    lineFirst = *value;
                }
                lineLast = *value;
            }
        }
        auto lineValue = lineFirst * 10 + lineLast;
        sumTwo += lineValue;
    }

    std::cout << sumOne << std::endl << sumTwo << std::endl;
}
