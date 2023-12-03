cmake_minimum_required(VERSION 3.17)
project(out)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)

add_subdirectory(utils)

add_executable(out main.cpp day1.hpp)
target_link_libraries(out utils)
target_include_directories(out PRIVATE $<TARGET_PROPERTY:utils,INTERFACE_INCLUDE_DIRECTORIES>)

set_target_properties(out PROPERTIES 
	VS_DEBUGGER_WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
	VS_DEBUGGER_COMMAND_ARGUMENTS "../../../io/example.input"
	CXX_STANDARD 20
	CXX_EXTENSIONS OFF)
set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT out)

if (WIN32)
	target_compile_definitions(out PUBLIC WIN32_LEAN_AND_MEAN)
	target_compile_definitions(out PUBLIC NOMINMAX)
endif ()
