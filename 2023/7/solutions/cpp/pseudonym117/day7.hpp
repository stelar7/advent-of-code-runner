#pragma once

#include <algorithm>
#include <cinttypes>
#include <fstream>
#include <string>
#include <vector>

namespace pseudonym117 {

enum class hand_strength {
  unknown = 0,
  high_card,
  one_pair,
  two_pair,
  three_of_a_kind,
  full_house,
  four_of_a_kind,
  five_of_a_kind
};

enum class card {
  unknown = 0,
  two = 2,
  three,
  four,
  five,
  six,
  seven,
  eight,
  nine,
  ten,
  jack,
  queen,
  king,
  ace
};

enum class rule_set {
  standard,
  jacks_are_wild,
};

struct play {
  card hand[5] {};
  int bet = -1;

  explicit operator bool() const {
    return bet != -1;
  }

  hand_strength strength(rule_set rules) const {
    auto rule_index = static_cast<int>(rules);
    if (strength_cache[rule_index] != hand_strength::unknown) {
      return strength_cache[rule_index];
    }

    char card_counts[static_cast<int>(card::ace)] {};

    for (auto c : hand) {
      card_counts[static_cast<int>(c)]++;
    }

    char wild_cards = 0;
    if (rules == rule_set::jacks_are_wild) {
      wild_cards = card_counts[static_cast<int>(card::jack)];
      card_counts[static_cast<int>(card::jack)] = 0;
    }

    std::sort(card_counts, card_counts + (static_cast<int>(card::ace) + 1), std::greater<>());

    hand_strength str = hand_strength::unknown;

    if (card_counts[0] + wild_cards == 5) {
      str = hand_strength::five_of_a_kind;
    } else if (card_counts[0] + wild_cards == 4) {
      str = hand_strength::four_of_a_kind;
    } else if (card_counts[0] + wild_cards == 3) {
      if (card_counts[1] == 2) {
        str = hand_strength::full_house;
      } else {
        str = hand_strength::three_of_a_kind;
      }
    } else if (card_counts[0] + wild_cards == 2) {
      if (card_counts[1] == 2) {
        str = hand_strength::two_pair;
      } else {
        str = hand_strength::one_pair;
      }
    } else if (card_counts[0] == 1) {
      str = hand_strength::high_card;
    }

    auto self = const_cast<play*>(this);
    self->strength_cache[rule_index] = str;
    return str;
  }
private:
  hand_strength strength_cache[static_cast<int>(rule_set::jacks_are_wild) + 1]{};
};

constexpr int card_rank(card c, rule_set rules) {
  switch (rules) {
    case rule_set::standard:
      return static_cast<int>(c);
    case rule_set::jacks_are_wild:
      if (c == card::jack) {
        return card_rank(card::two, rule_set::standard) - 1;
      } else {
        return card_rank(c, rule_set::standard);
      }
    default:
      throw std::exception();
  }
}

auto play_comparer(rule_set rules) {
  return [rules](const play& one, const play& two) {
    auto strength_cmp = one.strength(rules) <=> two.strength(rules);
    if (strength_cmp == std::strong_ordering::equal) {
      for (int i = 0; i < 5; ++i) {
        auto card_cmp = card_rank(one.hand[i], rules) <=> card_rank(two.hand[i], rules);
        if (card_cmp != std::strong_ordering::equal) {
          return card_cmp;
        }
      }
      return std::strong_ordering::equal;
    } else {
      return strength_cmp;
    }
  };
}

struct card_remapper {
  constexpr card map(char c) {
    auto ind = c - remap_const;
    if (ind < 0 || ind >= map_size) {
      return card::unknown;
    }
    return card_map[ind];
  }
private:
  static const int remap_const = '2';
  static const int map_size = 'T' - remap_const + 1;
  static constexpr std::array<card, map_size> build_map() {
    std::array<card, map_size> arr {};
    arr['2' - remap_const] = card::two;
    arr['3' - remap_const] = card::three;
    arr['4' - remap_const] = card::four;
    arr['5' - remap_const] = card::five;
    arr['6' - remap_const] = card::six;
    arr['7' - remap_const] = card::seven;
    arr['8' - remap_const] = card::eight;
    arr['9' - remap_const] = card::nine;
    arr['T' - remap_const] = card::ten;
    arr['J' - remap_const] = card::jack;
    arr['Q' - remap_const] = card::queen;
    arr['K' - remap_const] = card::king;
    arr['A' - remap_const] = card::ace;
    return arr;
  }

  const decltype(build_map()) card_map = build_map();
};

card_remapper remapper;

std::istream& operator>>(std::istream& in, play& pl) {
  std::string hand;
  in >> hand;

  if (hand.length() == 0) {
    return in;
  }
  if (hand.length() != 5) {
    throw std::exception();
  }

  for (auto i = 0; i < 5; ++i) {
    pl.hand[i] = remapper.map(hand[i]);
  }

  in >> pl.bet;
  return in;
}

std::istream& operator>>(std::istream& in, std::vector<play>& plays) {
  while (in) {
    play next;
    in >> next;
    if (next) {
      plays.emplace_back(next);
    }
  }
  return in;
}

template<typename Reporter>
struct day7 {
    void operator()(const char* filename) const {
        std::ifstream input(filename);

        std::vector<play> plays;
        input >> plays;

        for (auto rules : {rule_set::standard, rule_set::jacks_are_wild}) {
          auto comparer = play_comparer(rules);

          std::sort(plays.begin(), plays.end(),
            [&comparer](auto& one, auto& two) {
              return comparer(one, two) == std::strong_ordering::less;
            }
          );

          std::int64_t total_winnings = 0;
          for (auto i = 0; i < plays.size(); ++i) {
            std::int64_t rank = i + 1;
            total_winnings += rank * plays[i].bet;
          }

          Reporter::report(total_winnings);
        }
    }
};
}
