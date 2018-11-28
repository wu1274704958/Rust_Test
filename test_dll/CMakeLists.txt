cmake_minimum_required(VERSION 3.0.0)
project(test_dll VERSION 0.1.0)

set(BUILD_USE_64BITS on)
add_library(test_dll SHARED test_dll.cpp)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
