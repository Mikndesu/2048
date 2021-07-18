cmake_minimum_required(VERSION 3.5)
project(2048)
add_executable(2048 main.cpp)

set(CMAKE_CXX_COMPILER clang++)
if(WIN32)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /std:c++latest")
set(CMAKE_SYSTEM_PROCESSOR x64)
endif()
if(UNIX)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++2a")
add_definitions(-Wall -Wextra -O2)
endif()

add_subdirectory(libs)

# target_link_libraries(download_vimeo vimeocore)
