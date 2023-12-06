#pragma once

#include <algorithm>
#include <fstream>
#include <numeric>
#include <string>
#include <utility>
#include <vector>

namespace pseudonym117 {
struct location {
    int x;
    int y;
    int length;
};

struct part {
    location loc;
    char part;
    std::vector<int> part_numbers;
};

void parse_line(int line_y, const std::string_view& line, std::vector<location>& part_locations, std::vector<location>& part_number_locations) {
    int part_number_start = -1;

    auto check_part_number_state = [&](int x) {
        if (part_number_start != -1) {
            auto length = x - part_number_start;
            auto number_str = line.substr(part_number_start, length);
            auto number = static_cast<int>(parse_unsigned_int_nocheck(number_str));
            part_number_locations.push_back(location{part_number_start, line_y, length});

            part_number_start = -1;
        }
    };

    for (auto x = 0; x < line.size(); ++x) {
        char c = line[x];

        if ('0' <= c && c <= '9') {
            if (part_number_start == -1) {
                part_number_start = x;
            }
            continue;
        }
        check_part_number_state(x);
        if (c == '.') {   
            continue;
        }
        part_locations.push_back(location{x, line_y, 1});
    }

    check_part_number_state(line.size());
}

std::vector<part> parse_parts(std::istream& input) {
    std::vector<location> part_locations;

    std::vector<std::string> lines;
    std::vector<std::vector<location>> part_number_locations_by_line;
    std::string line;
    int y = 0;
    while (std::getline(input, line)) {
        lines.push_back(line);
        part_number_locations_by_line.push_back(std::vector<location>());
        parse_line(y, line, part_locations, *part_number_locations_by_line.rbegin());
        ++y;
    }

    std::vector<part> parts;
    for (auto& part_loc : part_locations) {
        std::vector<location> part_numbers;
        auto is_adjacent = [&part_loc](auto& number_location) {
            return part_loc.x >= number_location.x - 1
                && part_loc.x <= number_location.x + number_location.length
                && part_loc.y >= number_location.y - 1
                && part_loc.y <= number_location.y + 1;
        };
        auto inserter = std::back_inserter(part_numbers);

        if (part_loc.y > 0) {
            auto& above = part_number_locations_by_line[part_loc.y - 1];
            std::copy_if(above.begin(), above.end(), inserter, is_adjacent);
        }
        auto& same = part_number_locations_by_line[part_loc.y];
        std::copy_if(same.begin(), same.end(), inserter, is_adjacent);
        if (part_loc.y < part_number_locations_by_line.size() - 1) {
            auto& next = part_number_locations_by_line[part_loc.y + 1];
            std::copy_if(next.begin(), next.end(), inserter, is_adjacent);
        }
        if (part_numbers.empty()) {
            throw std::exception();
        }

        part p;
        p.loc = part_loc;
        p.part = lines[part_loc.y][part_loc.x];
        std::transform(part_numbers.begin(), part_numbers.end(), std::back_inserter(p.part_numbers),
            [&lines](auto& part_number_loc) {
                std::string_view line_view = lines[part_number_loc.y];
                auto number_view = line_view.substr(part_number_loc.x, part_number_loc.length);
                return static_cast<int>(parse_unsigned_int_nocheck(number_view));
            }
        );
        parts.push_back(p);
    }
    return parts;
}

template<typename Reporter>
struct day3 {
    void operator()(const char* filename) {
        std::ifstream input(filename);
        
        auto parts = parse_parts(input);

        auto part_sum = std::accumulate(parts.begin(), parts.end(), 0,
            [](auto existing, auto& part) {
                return existing + std::accumulate(part.part_numbers.begin(), part.part_numbers.end(), 0);
            }
        );

        std::vector<part> gears;
        std::copy_if(parts.begin(), parts.end(), std::back_inserter(gears),
            [](auto& part) {
                return part.part == '*' && part.part_numbers.size() == 2;
            }
        );

        auto gear_ratio_sum = std::accumulate(gears.begin(), gears.end(), 0,
            [](auto running_sum, auto& gear) {
                return running_sum + gear.part_numbers[0] * gear.part_numbers[1];
            }
        );

        Reporter::report(part_sum);
        Reporter::report(gear_ratio_sum);
    }
};
}
