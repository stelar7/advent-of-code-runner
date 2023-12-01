#include <chrono>

#include "day1.hpp"
#include "util.hpp"

using namespace pseudonym117;

int main(int argc, char* argv[]) {
    if (argc < 2) {
        return -1;
    }
    auto filename = argv[1];

    bool bench = argc > 2 && argv[2] == std::string("--bench");

    if (bench) {
        const int iterations = 10000;
        auto start = std::chrono::high_resolution_clock::now();
        for (auto i = 0; i < iterations; ++i) {
            day1<noop_reporter>(filename);
        }
        auto end = std::chrono::high_resolution_clock::now();

        auto duration = end - start;
        auto runtime = std::chrono::duration_cast<std::chrono::milliseconds>(duration);
        std::cout << "total time: " << runtime << ", average: " << runtime / (double)iterations << std::endl;
    } else {
        day1<stdout_reporter>(filename);
    }
}
