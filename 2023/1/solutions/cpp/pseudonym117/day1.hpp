#pragma once

#include <fstream>
#include <optional>
#include <string>
#include <limits>
#include <memory>

namespace pseudonym117 {

enum TrieMatchType {
    none,
    partial,
    full,
};

class TrieNode;

static const int CHAR_MAP_BUCKETS = 'z' - '0' + 1;

template<typename Value>
class char_map {
    bool _empty = true;
    std::unique_ptr<Value> buckets[CHAR_MAP_BUCKETS];
public:
    char_map() {}

    char_map(const char_map&) = delete;
    char_map& operator=(const char_map&) = delete;

    bool empty() const { return _empty; }
    bool contains(char c) const {
        auto index = c - '0';
        if (index < 0 || index >= CHAR_MAP_BUCKETS) {
            return false;
        }
        return static_cast<bool>(buckets[index]);
    }

    const Value& at(char c) const {
        auto index = c - '0';
        if (index < 0 || index >= CHAR_MAP_BUCKETS) {
            throw std::out_of_range("illegal index");
        }
        
        auto& toReturn = buckets[index];
        if (!toReturn) {
            throw std::out_of_range("not in map");
        }
        return *toReturn;
    }

    Value& operator[](char c) {
        auto index = c - '0';
        if (index < 0 || index >= CHAR_MAP_BUCKETS) {
            throw std::out_of_range("illegal index");
        }

        auto& existing = buckets[index];
        if (existing) {
            return *existing;
        }

        _empty = false;
        buckets[index] = std::make_unique<Value>();
        return *buckets[index];
    }
};

struct TrieMatch {
    const TrieMatchType type;
    const std::optional<int> value;
    const TrieNode* continue_at = nullptr;

    constexpr TrieMatch() : type(TrieMatchType::none) {}
    constexpr TrieMatch(const TrieNode* continue_at) : type(TrieMatchType::partial), continue_at(continue_at) {}
    constexpr TrieMatch(const int value) : type(TrieMatchType::full), value(value) {}
};

class TrieNode {
    int value = std::numeric_limits<int>::min();
    char_map<TrieNode> children;
public:
    TrieNode() {}
    TrieNode(const TrieNode&) = delete;
    TrieNode& operator=(const TrieNode&) = delete;

    operator bool() const { return value != std::numeric_limits<int>::min(); }

    const TrieMatch at(const std::string_view& key) const {
        if (key.empty()) {
            if (*this) {
                return value;
            }
            if (!children.empty()) {
                return TrieMatch(this);
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

    const TrieNode& get_root() const { return root; }

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

template<typename Reporter>
int day1(char* filename) {
    std::ifstream file(filename);

    const Trie lookup{
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
            auto currentNode = &lookup.get_root();
            for (auto scanIndex = 0; scanIndex < line.size(); ++scanIndex) {
                std::string_view current(line.c_str() + i + scanIndex, 1);

                auto match = currentNode->at(current);

                if (match.type == TrieMatchType::none) {
                    break;
                }
                if (match.type == TrieMatchType::partial) {
                    currentNode = match.continue_at;
                    continue;
                }
                if (match.type == TrieMatchType::full) {
                    value = match.value;
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

    Reporter::report(sumOne);
    Reporter::report(sumTwo);
    
    return 0;
}
}
