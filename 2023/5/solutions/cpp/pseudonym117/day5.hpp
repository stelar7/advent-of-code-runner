#pragma once

#include <algorithm>
#include <fstream>
#include <functional>
#include <cstdint>
#include <limits>
#include <numeric>
#include <optional>
#include <string>
#include <sstream>
#include <vector>

namespace pseudonym117 {

typedef std::int64_t iint;

struct range {
    iint start;
    iint length;
};

struct remap {
    iint destination_start = -1;
    iint source_start = -1;
    iint range_length = -1;

    explicit operator bool() const {
        return destination_start != -1 && source_start != -1 && range_length != -1;
    }
};

auto operator<=>(const remap& first, const remap& second) {
    return first.source_start <=> second.source_start;
}

auto operator<=>(const remap& remap, iint value) {
    if (value < remap.source_start) {
        return std::partial_ordering::less;
    } else if (value < remap.source_start + remap.range_length) {
        return std::partial_ordering::equivalent;
    } else {
        return std::partial_ordering::greater;
    }
}

struct almanac {
    std::vector<iint> seed_input;
    std::vector<remap> seed_to_soil;
    std::vector<remap> soil_to_fertilizer;
    std::vector<remap> fertilizer_to_water;
    std::vector<remap> water_to_light;
    std::vector<remap> light_to_temperature;
    std::vector<remap> temperature_to_humidity;
    std::vector<remap> humidity_to_location;

    std::vector<range> location_for_seeds(const range& r) {
        auto current_ranges = std::vector<range>{ r };

        for (auto mappings : {
            &seed_to_soil,
            &soil_to_fertilizer,
            &fertilizer_to_water,
            &water_to_light,
            &light_to_temperature,
            &temperature_to_humidity,
            &humidity_to_location,
        }) {
            current_ranges = adjust_range(*mappings, current_ranges);
        }

        return current_ranges;
    }
private:
    static std::vector<range> adjust_range(const std::vector<remap>& mappings, const std::vector<range>& ranges) {
        using ord = std::partial_ordering;

        std::vector<range> res;

        for (auto& r : ranges) {
            auto [current_start, remaining_length] = r;

            for (auto i = 0; i < mappings.size() && remaining_length > 0; ++i) {
                auto& mapping = mappings[i];

                auto cmp = mapping <=> current_start;
                if (cmp == ord::less) {
                    auto next_start = current_start;
                    auto next_length = std::min(remaining_length, mapping.source_start - next_start);

                    res.push_back({ next_start, next_length });

                    remaining_length -= next_length;
                    current_start = next_start + next_length;
                }
                else if (cmp == ord::equivalent) {
                    auto next_start = mapping.destination_start + (current_start - mapping.source_start);
                    auto next_length = std::min(remaining_length, mapping.range_length - (current_start - mapping.source_start));

                    res.push_back({ next_start, next_length });

                    remaining_length -= next_length;
                    current_start = current_start + next_length;
                }
                else {
                    continue;
                }
            }

            if (remaining_length > 0) {
                res.push_back({ current_start, remaining_length });
            }
        }

        return res;
    }
};

std::istream& operator>>(std::istream& in, std::vector<iint>& values) {
    std::string line;
    std::getline(in, line);

    std::stringstream stream(line);

    while (stream) {
        iint next = std::numeric_limits<iint>::min();
        stream >> next;

        if (next != std::numeric_limits<iint>::min()) {
            values.push_back(next);
        } else {
            break;
        }
    }

    return in;
}

std::istream& operator>>(std::istream& in, remap& remap) {
    std::string line;
    std::getline(in, line);

    std::stringstream stream(line);
    stream >> remap.destination_start;
    stream >> remap.source_start;
    stream >> remap.range_length;

    return in;
}

std::istream& operator>>(std::istream& in, std::vector<remap>& values) {
    while (in) {
        remap next;
        in >> next;
        if (next) {
            values.push_back(next);
        } else {
            break;
        }
    }

    std::sort(values.begin(), values.end());

    return in;
}

void advance_to_next_filled_line(std::istream& in) {
    char c = in.peek();
    while (c == '\n') {
        in.get();
        c = in.peek();
    }
}

std::istream& operator>>(std::istream& in, almanac& alm) {
    std::string unused;
    in >> unused;
    in >> alm.seed_input;

    advance_to_next_filled_line(in);
    std::getline(in, unused);
    in >> alm.seed_to_soil;

    advance_to_next_filled_line(in);
    std::getline(in, unused);
    in >> alm.soil_to_fertilizer;

    advance_to_next_filled_line(in);
    std::getline(in, unused);
    in >> alm.fertilizer_to_water;

    advance_to_next_filled_line(in);
    std::getline(in, unused);
    in >> alm.water_to_light;

    advance_to_next_filled_line(in);
    std::getline(in, unused);
    in >> alm.light_to_temperature;

    advance_to_next_filled_line(in);
    std::getline(in, unused);
    in >> alm.temperature_to_humidity;

    advance_to_next_filled_line(in);
    std::getline(in, unused);
    in >> alm.humidity_to_location;

    return in;
}

template<typename Reporter>
struct day5 {
    void operator()(const char* filename) const {
        std::ifstream input(filename);

        almanac alm;

        input >> alm;

        std::vector<range> seed_locations;
        for (auto seed : alm.seed_input) {
            auto locations = alm.location_for_seeds({seed, 1});
            for (auto& r : locations) {
                seed_locations.push_back(r);
            }
        }

        auto min_location = std::accumulate(
            seed_locations.begin(),
            seed_locations.end(),
            std::numeric_limits<iint>::max(),
            [](auto acc, auto& val) { return std::min(acc, val.start); }
        );

        std::vector<range> seed_ranges;
        for (auto i = 1; i < alm.seed_input.size(); i += 2) {
            seed_ranges.push_back({alm.seed_input[i - 1], alm.seed_input[i]});
        }

        std::vector<range> location_ranges;
        for (auto& r : seed_ranges) {
            auto locations = alm.location_for_seeds(r);
            for (auto& loc : locations) {
                location_ranges.push_back(loc);
            }
        }

        auto min_location_range = std::accumulate(
            location_ranges.cbegin(),
            location_ranges.cend(),
            std::numeric_limits<iint>::max(),
            [&alm](iint acc, const range& r) {
                return std::min(acc, r.start);
            }
        );

        Reporter::report(min_location);
        Reporter::report(min_location_range);
    }
};
}
