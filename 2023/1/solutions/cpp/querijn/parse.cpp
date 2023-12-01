#include "parse.hpp"

#include <fstream>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <array>
#include <algorithm>

uint64_t result1 = 0;
uint64_t result2 = 0;

#define DATA_BUFFER_SIZE 23000
static char data_buffer[DATA_BUFFER_SIZE];

void get_file(const char* file_name, char*& data, char*& end_data)
{
	data = data_buffer;

	std::ifstream input(file_name, std::ios::binary);
	if (!input.is_open())
		return;

	input.seekg(0, input.end);
	uint64_t len = input.tellg();
	assert(len + 8 < DATA_BUFFER_SIZE); // + 8 because of OOB integer reads
	end_data = data + len;

	input.seekg(0, input.beg);
	input.read(data, len);
}

void read_int(char*& input, int& n)
{
	n = *input - '0';
	input++;
	while (*input >= '0' && *input <= '9')
	{
		n = 10 * n + *input - '0';
		input++;
	}
}

void read_char_int(char*& input, int& n)
{
	n = *input - '0';
	input++;
}

void eat_non_number(char*& input, char* end)
{
	while (input < end && *input < '0' || *input > '9')
		input++;
}

void eat_non_number_line(char*& input, char* end)
{
	while (input < end && *input != '\n' && (*input < '0' || *input > '9'))
		input++;
}

bool peek(const char* input, const char* needle)
{
	return strstr(input, needle) == input;
}

#include <chrono>

void func(const char* file_name);

#if BENCHMARK

int main(int argc, char* argv[])
{
	constexpr int runs = 1500;
	auto start = std::chrono::steady_clock::now();
	for (int i = 0; i < runs; i++)
		func(argv[1]);
	auto end = std::chrono::steady_clock::now();
	double time_taken = std::chrono::duration<double>(end - start).count();
	printf("Total: %3.2f sec\nAvg: %5.3f ms per run\n\n", time_taken, (time_taken / (double)runs) * 1000.0);

	std::cout << result1 << std::endl;
	std::cout << result2 << std::endl;
}

#else

int main(int argc, char* argv[])
{
	func(argv[1]);

	std::cout << result1 << std::endl;
	std::cout << result2 << std::endl;
	return 0;
}

#endif