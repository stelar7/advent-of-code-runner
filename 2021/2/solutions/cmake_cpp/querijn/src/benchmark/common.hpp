#pragma once

// TESTING sets the "test.txt" file as current file. If undefined, it uses "input.txt"
// #define TESTING 

// Do not edit below this line
#ifdef BENCHMARK
	#undef TESTING
	#undef ONE
	#undef TWO
	#undef RUNNER
#endif

#ifdef TESTING
	#define FILE_NAME "test.txt"

#else
	#define FILE_NAME "input.txt"

#endif
