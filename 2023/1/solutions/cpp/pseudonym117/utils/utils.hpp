#pragma once

#include <chrono>
#include <iostream>
#include <string>

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

template<template<typename> typename DayStruct>
struct main_runner {
    int operator()(int argc, const char* argv[]) const {
        if (argc < 2) {
            return -1;
        }
        auto filename = argv[1];

        bool bench = argc > 2 && argv[2] == std::string("--bench");

        if (bench) {
            DayStruct<noop_reporter> day;

            const int iterations = 10000;
            auto start = std::chrono::high_resolution_clock::now();
            for (auto i = 0; i < iterations; ++i) {
                day(filename);
            }
            auto end = std::chrono::high_resolution_clock::now();

            auto duration = end - start;
            auto runtime = std::chrono::duration_cast<std::chrono::milliseconds>(duration);
            std::cout << "total time: " << runtime << ", average: " << runtime / (double)iterations << std::endl;
        }
        else {
            DayStruct<stdout_reporter> day;
            day(filename);
        }

        return 0;
    }
};
}
