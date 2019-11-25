cmake_minimum_required(VERSION 3.10)
project(micromachines)

# Estándar C++11
set (CMAKE_CXX_STANDARD 11)
set (CMAKE_CXX_STANDARD_REQUIRED ON)
set (CMAKE_CXX_EXTENSIONS OFF) # -std=c++11 en lugar de -std=gnu++11

# Todos los #include son relativos a ejemplos
include_directories(ejemplos_facu)

# Archivos generados por CMake
include_directories(${PROJECT_BINARY_DIR}/ejemplos)

add_subdirectory(ejemplos_facu/Ownership)

add_executable(c_ownership_1 ejemplos_facu/Ownership/ownership_1.cpp)
add_executable(c_ownership_2 ejemplos_facu/Ownership/ownership_2.cpp)
