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
include_directories(${Qt5Widgets_INCLUDE_DIRS})

add_subdirectory(ejemplos/Ownership)

add_executable(c_ownership_1 ejemplos/Ownership/ownership_1.cpp)
