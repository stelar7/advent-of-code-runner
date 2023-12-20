#pragma once

#include <cinttypes>
#include <fstream>
#include <limits>
#include <string>
#include <sstream>
#include <utility>
#include <vector>

namespace pseudonym117 {

std::istream& operator>>(std::istream& in, std::vector<int>& values) {
    std::string line;
    if (std::getline(in, line)) {
        std::stringstream str(line);

        while (true) {
            auto val = std::numeric_limits<int>::min();
            str >> val;
            if (val != std::numeric_limits<int>::min()) {
                values.push_back(val);
            } else {
                break;
            }
        }
    }

    return in;
}

std::istream& operator>>(std::istream& in, std::vector<std::vector<int>>& sequences) {
    while (true) {
        std::vector<int> next;
        in >> next;

        if (next.empty()) {
            break;
        } else {
            sequences.emplace_back(std::move(next));
        }
    }

    return in;
}

std::pair<std::vector<int>, bool> diffs(const std::vector<int>& sequence) {
    if (sequence.size() < 2) {
        throw std::exception();
    }

    std::vector<int> res;
    auto all_zero = true;
    for (auto i = 1; i < sequence.size(); ++i) {
        auto diff = sequence[i] - sequence[i - 1];
        res.push_back(diff);
        all_zero &= diff == 0;
    }

    return std::make_pair(res, all_zero);
}

std::pair<int, int> extrapolated_value(const std::vector<int>& sequence) {
    auto [diff, all_zero] = diffs(sequence);
    if (all_zero) {
        return std::make_pair(*sequence.cbegin(), *sequence.crbegin());
    }
    auto [left, right] = extrapolated_value(diff);

    return std::make_pair(*sequence.cbegin() - left, *sequence.crbegin() + right);
}

template<typename Reporter>
struct day9 {
  void operator()(const char* filename) const {
    std::ifstream input(filename);

    std::vector<std::vector<int>> sequences;

    input >> sequences;

    std::int64_t previous_sum = 0, next_sum = 0;
    for (auto& sequence : sequences) {
        auto [previous, next] = extrapolated_value(sequence);
        previous_sum += previous;
        next_sum += next;
    }

    Reporter::report(next_sum);
    Reporter::report(previous_sum);
  }
};
}
