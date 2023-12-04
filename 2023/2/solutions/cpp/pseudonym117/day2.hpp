#pragma once

#include <algorithm>
#include <fstream>
#include <numeric>
#include <vector>

namespace pseudonym117 {
struct PullResults {
    int red;
    int green;
    int blue;

    int power() const {
        return red * green * blue;
    }
};

// maybe can do in place, but tbh i doubt it matters much
PullResults required(const PullResults& first, const PullResults& second) {
    return PullResults{
        std::max(first.red, second.red),
        std::max(first.green, second.green),
        std::max(first.blue, second.blue)
    };
}

struct DiceGame {
    int id = -1;
    std::vector<PullResults> pulls;

    bool is_valid() const { return id != -1; }

    PullResults required_dice() const {
        return std::accumulate(pulls.begin(), pulls.end(), PullResults(), required);
    }
};

std::istream& operator>>(std::istream& in, PullResults& results) {
    int i;
    std::string str;

    results.red = results.green = results.blue = 0;

    char last = ',';

    do {
        in >> i;
        in >> str;

        if (str.empty()) {
            break;
        }

        last = *str.rbegin();
        std::string_view color;
        if (last == ',' || last == ';') {
            color = std::string_view(str.c_str(), str.length() - 1);
        }
        else {
            color = str;
        }

        if (color == "red") {
            results.red = i;
        }
        else if (color == "green") {
            results.green = i;
        }
        else if (color == "blue") {
            results.blue = i;
        }
        else {
            throw std::out_of_range("encountered illegal color");
        }
    } while (last == ',');
    

    return in;
}

std::istream& operator>>(std::istream& in, DiceGame& game) {
    std::string str;
    int i;

    // Game
    in >> str;
    in >> game.id;

    // :
    in >> str;

    for (char c = in.peek(); c != '\n' && c != std::char_traits<char>::eof(); c = in.peek()) {
        PullResults results;
        in >> results;
        game.pulls.push_back(results);
    }

    return in;
}

template<typename Reporter>
struct day2 {
	void operator()(const char* filename) const {
        std::ifstream input(filename);

        std::vector<DiceGame> games;
        while (input) {
            DiceGame game;
            input >> game;
            if (game.is_valid()) {
                games.push_back(game);
            }
        }

        const int available_red = 12;
        const int available_green = 13;
        const int available_blue = 14;
        std::vector<DiceGame> valid_games;

        std::copy_if(games.begin(), games.end(), std::back_inserter(valid_games),
            [](const DiceGame& game) {
                return std::all_of(game.pulls.begin(), game.pulls.end(),
                    [](const PullResults& pull) {
                        return pull.red <= available_red
                            && pull.green <= available_green
                            && pull.blue <= available_blue;
                    }
                );
            }
        );

        auto id_sum = std::accumulate(valid_games.begin(), valid_games.end(), 0,
            [](int existing, const DiceGame& next) {
                return existing + next.id;
            }
        );

        auto total_power_of_required = std::accumulate(games.begin(), games.end(), 0,
            [](int existing_power, const DiceGame& game) {
                auto game_power = game.required_dice().power();
                return existing_power + game_power;
            }
        );

        Reporter::report(id_sum);
        Reporter::report(total_power_of_required);
	}
};
}
