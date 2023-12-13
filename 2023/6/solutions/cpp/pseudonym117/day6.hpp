#pragma once

#include <algorithm>
#include <cmath>
#include <cinttypes>
#include <fstream>
#include <limits>
#include <numeric>
#include <ranges>
#include <sstream>
#include <string>
#include <vector>

namespace pseudonym117 {

typedef std::int64_t iint;

std::istream& operator>>(std::istream& in, std::vector<iint>& values) {
    std::string line, unused;
    std::getline(in, line);
    std::stringstream times_line(line);
    times_line >> unused;
    while(times_line) {
        iint value = std::numeric_limits<iint>::min();
        times_line >> value;
        if (value != std::numeric_limits<iint>::min()) {
            values.push_back(value);
        }
    }
    return in;
}

iint text_combine(const std::vector<iint>& values) {
  std::stringstream buf;
  for (auto value : values) {
    buf << value;
  }

  buf.seekg(0, std::ios::beg);

  iint combined;
  buf >> combined;
  return combined;
}

iint ways_to_beat(iint race_time, iint record_distance) {
  // d = (race_time - charge_time) * charge_time
  //
  // record_distance = (race_time - charge_time) * charge_time
  // r = (t - x) * x
  // r = tx - x^2
  // x^2 - tx + r = 0
  // x = t +- rt( (-t)^2 - 4 * 1 * r) / (2 * 1)
  // x = t +- rt( t^2 - 4r) / 2

  auto plus = (race_time + std::sqrt((race_time * race_time) - (4 * record_distance))) / 2;
  auto minus = (race_time - std::sqrt((race_time * race_time) - (4 * record_distance))) / 2;

  // shamelessly copy stelar to ensure that we do not count ties
  auto lower = static_cast<iint>(std::ceil(std::min(plus, minus) + 0.001));
  auto upper = static_cast<iint>(std::floor(std::max(plus, minus) - 0.001));

  return upper - lower + 1;
}

template<typename Reporter>
struct day6 {
    void operator()(const char* filename) const {
        std::ifstream input(filename);

        std::vector<iint> times;
        std::vector<iint> distances;

        input >> times;
        input >> distances;

        auto records = std::views::zip(times, distances);

        std::vector<iint> possible_wins;
        for (auto [race_time, record_distance] : records) {
          possible_wins.push_back(ways_to_beat(race_time, record_distance));
        }

        auto margin_of_error = std::reduce(possible_wins.begin(), possible_wins.end(), 1,
            [](auto a, auto b) { return a * b; }
        );

        auto large_race_time = text_combine(times);
        auto large_race_distance = text_combine(distances);

        auto large_margin_of_error = ways_to_beat(large_race_time, large_race_distance);

        Reporter::report(margin_of_error);
        Reporter::report(large_margin_of_error);
    }
};
}
