cmake_minimum_required(VERSION 3.17)
project(utils)

add_library(utils INTERFACE utils.hpp)
target_include_directories(utils INTERFACE .)
