cmake_minimum_required(VERSION 3.17)
project(out)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)
set(SOURCES "" CACHE INTERNAL "Source Files")
function(ADD_SRC var folder inc src)
	source_group(${folder} FILES ${inc} ${src})
	set(${var} ${${var}} ${inc} ${src} CACHE INTERNAL "")
endfunction(ADD_SRC)

ADD_SRC(SOURCES	"Benchmark"		"src/benchmark/debugbreak.h"		"")
ADD_SRC(SOURCES	"Benchmark"		"src/benchmark/duration.hpp"		"src/benchmark/duration.cpp")
ADD_SRC(SOURCES	"Benchmark"		""									"src/benchmark/benchmark.cpp")

ADD_SRC(SOURCES	"Common"		"src/benchmark/common.hpp"			"")
ADD_SRC(SOURCES	"Code"			"src/one.cpp"						"src/two.cpp")
ADD_SRC(SOURCES	"Input"			"test.txt"							"input.txt")

add_executable(out ${SOURCES})
target_include_directories(out PUBLIC "src" "src/benchmark")
set_target_properties(out PROPERTIES 
	VS_DEBUGGER_WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
	CXX_STANDARD 20
	CXX_EXTENSIONS OFF)
set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT out)

if (WIN32)
	target_compile_definitions(out PUBLIC WIN32_LEAN_AND_MEAN)
	target_compile_definitions(out PUBLIC NOMINMAX)
endif ()

# target_compile_definitions(out PUBLIC TESTING)

# Set the main entry point below (ONE, TWO, RUNNER or BENCHMARK)
# target_compile_definitions(out PUBLIC ONE=main)
# target_compile_definitions(out PUBLIC TWO=main)
target_compile_definitions(out PUBLIC RUNNER=main)
# target_compile_definitions(out PUBLIC BENCHMARK=main)
