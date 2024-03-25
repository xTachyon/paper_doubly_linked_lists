cmake_minimum_required(VERSION 3.21)
project(cpp_tests)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

add_library(cpp_tests SHARED lib.cpp)
