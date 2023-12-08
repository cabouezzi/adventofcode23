cmake_minimum_required(VERSION 3.26)
project(day1)

set(CMAKE_CXX_STANDARD 17)

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/input.txt ${CMAKE_CURRENT_BINARY_DIR} COPYONLY)

add_executable(day1 main.cpp)
