cmake_minimum_required (VERSION 3.0)
cmake_policy(VERSION 3.0)

project(main)

set(CMAKE_CXX_STANDARD 17) # this does nothing for MSVC, use target_compile_options below
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS ON)

add_executable(main main.cpp)

