cmake_minimum_required(VERSION 3.17)
project(out)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)

add_executable(out main.cpp parse.hpp parse.cpp)
set_target_properties(out PROPERTIES 
	VS_DEBUGGER_WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
	CXX_STANDARD 20
	CXX_EXTENSIONS OFF)
set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT out)

if (WIN32)
	target_compile_definitions(out PUBLIC WIN32_LEAN_AND_MEAN)
	target_compile_definitions(out PUBLIC NOMINMAX)
else ()
	SET(CMAKE_C_COMPILER gcc-11)
endif ()