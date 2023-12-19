#pragma once

#include <algorithm>
#include <concepts>
#include <cinttypes>
#include <fstream>
#include <numeric>
#include <ranges>
#include <string>
#include <unordered_map>
#include <vector>

namespace pseudonym117 {

typedef std::string key;

constexpr std::string as_key(const std::string_view& str) {
  return {str.data(), str.length()};
}

template<typename InputIt>
constexpr std::int64_t lcm(InputIt first, InputIt last) {
  std::int64_t acc = 0;

  auto iter = first;
  while (iter != last) {
    if (acc == 0) {
      acc = *iter;
      continue;
    }
    acc = std::lcm(acc, *iter);
    iter++;
  }

  return acc;
}

template<typename Value>
struct node {
  node() = default;
  explicit node(const Value& value) : _value(value) {}

  Value _value;
  node<Value>* left = nullptr;
  node<Value>* right = nullptr;
};

node<key>* get_or_add(std::unordered_map<key, node<key>>& node_map, const key& value) {
  if (!node_map.contains(value)) {
    node next(value);
    node_map[value] = next;
  }

  auto& existing = node_map[value];
  return &existing;
}

std::istream& operator>>(std::istream& in, std::unordered_map<key, node<key>>& node_map) {
  std::string line;
  while (std::getline(in, line)) {
    if (line.empty()) {
      continue;
    }

    // AAA = (BBB, CCC)
    std::string_view key_s(line.c_str(), 3);
    std::string_view left_s(line.c_str() + 7, 3);
    std::string_view right_s(line.c_str() + 12, 3);

    auto key_v = as_key(key_s);
    auto left_v = as_key(left_s);
    auto right_v = as_key(right_s);

    auto key = get_or_add(node_map, key_v);
    auto left = get_or_add(node_map, left_v);
    auto right = get_or_add(node_map, right_v);

    key->left = left;
    key->right = right;
  }

  return in;
}

template<std::predicate<key> EndPredicate>
int distance_to(
    const std::unordered_map<key, node<key>>& node_map,
    const std::string_view& pathing,
    const key& start,
    const EndPredicate& end_predicate
) {
  if (!node_map.contains(start)) {
    throw std::exception();
  }

  int distance = 0;

  auto* current = &node_map.at(start);
  while (!end_predicate(current->_value)) {
    auto path_index = distance % pathing.size();
    auto direction = pathing[path_index];
    if (direction == 'L') {
      current = current->left;
    } else {
      current = current->right;
    }
    ++distance;
  }

  return distance;
}

template<std::predicate<key> StartPredicate, std::predicate<key> EndPredicate>
auto multi_path_distance(
    const std::unordered_map<key, node<key>>& node_map,
    const std::string_view& pathing,
    const StartPredicate& start_predicate,
    const EndPredicate& end_predicate
) {
  auto nearest_ends = node_map
      | std::views::filter([&start_predicate](auto& kv) { return start_predicate(kv.first); })
      | std::views::values
      | std::views::transform([&](auto& n) {
        return distance_to(node_map, pathing, n._value, end_predicate);
      });

  return lcm(nearest_ends.begin(), nearest_ends.end());
}

template<typename Reporter>
struct day8 {
  void operator()(const char* filename) const {
    std::ifstream input(filename);

    std::string directions;
    std::unordered_map<key, node<key>> nodes;

    input >> directions;
    input >> nodes;

    auto a_to_z = distance_to(
      nodes,
      directions,
      as_key("AAA"),
      [](auto k) { return k == as_key("ZZZ"); }
    );

    auto all_a_to_all_z = multi_path_distance(
      nodes,
      directions,
      [](const key& k) { return k.ends_with('A'); },
      [](const key& k) { return k.ends_with('Z'); }
    );

    Reporter::report(a_to_z);
    Reporter::report(all_a_to_all_z);
  }
};
}
