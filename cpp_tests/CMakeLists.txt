cmake_minimum_required(VERSION 3.21)
project(cpp_tests)

set (CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED On)
set(CMAKE_CXX_EXTENSIONS Off)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

set(sources
    src/lib.cpp
    src/solutions/solutions.hpp
    src/solutions/manual_list.cpp
    src/solutions/std_list.cpp
)

add_library(cpp_tests SHARED ${sources})

include(CheckIPOSupported)
check_ipo_supported(RESULT lto_supported OUTPUT error)

if (lto_supported)
    message(STATUS "IPO / LTO enabled")
    set_property(TARGET cpp_tests PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
else()
    message(STATUS "IPO / LTO not supported: <${error}>")
endif()
