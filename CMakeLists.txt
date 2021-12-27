cmake_minimum_required(VERSION 3.20)
project(AdventOfCode2021)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /O2")

add_executable(AdventOfCode2021
        src/day24.cpp)
