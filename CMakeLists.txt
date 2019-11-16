cmake_minimum_required(VERSION 3.10)
project(micromachines)

# Estándar C++11
set (CMAKE_CXX_STANDARD 11)
set (CMAKE_CXX_STANDARD_REQUIRED ON)
set (CMAKE_CXX_EXTENSIONS OFF) # -std=c++11 en lugar de -std=gnu++11

# Todos los #include son relativos a ejemplos
include_directories(ejemplos)

# Archivos generados por CMake
include_directories(${PROJECT_BINARY_DIR}/ejemplos)

add_subdirectory(ejemplos/Ownership)
add_subdirectory(ejemplos/Concurrency)

add_executable(c_ownership_1 ejemplos/Ownership/ownership_1.cpp)
add_executable(c_ownership_2 ejemplos/Ownership/ownership_2.cpp)
add_executable(c_concurrency_1 ejemplos/Concurrency/concurrency_1.cpp)

target_link_libraries(c_concurrency_1 pthread)
