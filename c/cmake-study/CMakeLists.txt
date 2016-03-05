cmake_minimum_required(VERSION 3.3)
project(studyc)

INCLUDE_DIRECTORIES(include)
SET(EXECUTABLE_OUTPUT_PATH ${studyc_SOURCE_DIR}/bin)
SET(LIBRARY_OUTPUT_PATH ${studyc_SOURCE_DIR}/lib)

set(SRC_DIR ./src/)
set(SOURCE_FILES ${SRC_DIR}main.c)
add_executable(studyc ${SOURCE_FILES})
