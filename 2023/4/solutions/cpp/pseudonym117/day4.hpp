#pragma once

#include <bitset>
#include <fstream>
#include <string>
#include <unordered_map>
#include <vector>

#include <utils.hpp>

namespace pseudonym117 {

std::vector<unsigned int> parse_numbers(const std::string_view& str) {
    std::vector<unsigned int> res;

    auto start_index = -1;
    for (auto i = 0; i < str.size(); ++i) {
        auto c = str[i];
        if ('0' <= c && c <= '9') {
            if (start_index == -1) {
                start_index = i;
            }
        } else if (start_index != -1) {
            auto number_str = str.substr(start_index, i - start_index);
            auto as_int = parse_unsigned_int_nocheck(number_str);
            res.push_back(as_int);
            start_index = -1;
        }
    }
    if (start_index != -1) {
        auto number_str = str.substr(start_index);
        auto as_int = parse_unsigned_int_nocheck(number_str);
        res.push_back(as_int);
    }

    return res;
}

auto to_bitset(const std::vector<unsigned int>& numbers) {
    std::bitset<128> res;

    for (auto& n : numbers) {
        res.set(n);
    }

    return res;
}

int total_cards_from_card(
    const std::vector<unsigned int>& matches_per_card,
    int card_number,
    std::unordered_map<int, int>& card_number_to_prizes) {
    if (card_number_to_prizes.contains(card_number)) {
        return card_number_to_prizes[card_number];
    }

    int prizes = 1;
    for (auto i = 1; i <= matches_per_card[card_number]; ++i) {
        prizes += total_cards_from_card(matches_per_card, card_number + i, card_number_to_prizes);
    }
    card_number_to_prizes[card_number] = prizes;
    return prizes;
}

int total_cards_from_card(
    const std::vector<unsigned int>& matches_per_card,
    int card_number) {
    std::unordered_map<int, int> cache;
    return total_cards_from_card(matches_per_card, card_number, cache);
}

template<typename Reporter>
struct day4 {
    void operator()(const char* filename) {
        std::ifstream input(filename);

        std::vector<unsigned int> matches_per_card;

        std::string line;
        while (std::getline(input, line)) {
            const char* winning_start;
            const char* actual_start;
            for (auto i = 0; i < line.size(); ++i) {
                auto loc = line.c_str() + i;
                if (*loc == ':') {
                    winning_start = loc + 2;
                }
                if (*loc == '|') {
                    actual_start = loc + 2;
                    break;
                }
            }

            std::string_view winning(winning_start, actual_start - 3);
            std::string_view actual(actual_start);

            auto winning_numbers = parse_numbers(winning);
            auto actual_numbers = parse_numbers(actual);

            auto winning_bitset = to_bitset(winning_numbers);
            auto actual_bitset = to_bitset(actual_numbers);

            auto matches = winning_bitset & actual_bitset;

            auto matching_numbers = matches.count();
            matches_per_card.push_back(matching_numbers);
        }

        int total_points = 0;
        for (auto i = 0; i < matches_per_card.size(); ++i) {
            auto matching_numbers = matches_per_card[i];
            if (matching_numbers > 0) {
                auto card_points = std::pow(2, matching_numbers - 1);
                total_points += card_points;
            }
        }

        int total_cards = 0;
        std::unordered_map<int, int> card_number_to_prizes;
        for (auto i = 0; i < matches_per_card.size(); ++i) {
            auto prizes = total_cards_from_card(matches_per_card, i);
            total_cards += prizes;
        }

        Reporter::report(total_points);
        Reporter::report(total_cards);
    }
};
}
