#pragma once

#include <iostream>

namespace pseudonym117 {
struct stdout_reporter {
    template<typename Arg>
    static void report(const Arg& arg) {
        std::cout << arg << std::endl;
    }
};

struct noop_reporter {
    template<typename Arg>
    static void report(const Arg& arg) {}
};
}
