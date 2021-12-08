#define BENCHMARK 0

#pragma region COMMON
#include <fstream>
#include <cassert>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <array>
#include <algorithm>

uint64_t result1 = 0;
uint64_t result2 = ~0;

#if defined(_DEBUG)
#define ASS(input) assert(input)
#else
#define ASS(input) ((void)0)
#endif

#if defined(_MSC_VER)
#define FORCE_INLINE __forceinline
#elif defined(__GNUC__)
#define FORCE_INLINE inline __attribute__((always_inline))
#elif (__STDC_VERSION__ >= 199901L)
#define FORCE_INLINE INLINE
#else
#define FORCE_INLINE INLINE
#endif

#define DATA_BUFFER_SIZE 20000
char data_buffer[DATA_BUFFER_SIZE];

FORCE_INLINE void get_file(const char* file_name, char*& data, char*& end_data)
{
	data = data_buffer;

	std::ifstream input(file_name, std::ios::binary);
	if (!input.is_open())
		return;

	input.seekg(0, input.end);
	uint64_t len = input.tellg();
	ASS(len < DATA_BUFFER_SIZE);
	end_data = data + len;

	input.seekg(0, input.beg);
	input.read(data, len);
}

FORCE_INLINE void read_int(char*& input, int& n)
{
	n = *input - '0';
	input++;
	while (*input >= '0' && *input <= '9')
	{
		n = 10 * n + *input - '0';
		input++;
	}
}

FORCE_INLINE void read_char_int(char*& input, int& n)
{
	n = *input - '0';
	input++;
}

FORCE_INLINE void eat_non_number(char*& input, char* end)
{
	while (input < end && *input < '0' || *input > '9')
		input++;
}

#include <chrono>

FORCE_INLINE int func(const char* file_name);
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
#pragma endregion

// #define FILE_NAME "example.txt"
#define FILE_NAME "input.txt"

constexpr int max_pos = 2000;
bool first = 1;

constexpr auto cost2_lut = []
{
	std::array<uint64_t, max_pos> array = {};

	for (int i = 0; i < max_pos; ++i)
		array[i] = (i * (i + 1)) >> 1;

	return array;
}();

constexpr auto abs_lut = []
{
	std::array<uint64_t, max_pos * 2> array = {};

	for (int i = -max_pos; i < max_pos; ++i)
		array[i + max_pos] = i < 0 ? -i : i;

	return array;
}();

int counts[max_pos];

FORCE_INLINE int median(int* numbers, int array_size)
{
#if BENCHMARK
	memset(counts, 0, sizeof(int) * array_size);
#endif

	// Initial part of a count sort
	for (int i = 0; i < array_size; i++)
		counts[numbers[i]]++;
	
	// lmao who the fuck sorts amirite fellas
	int intermediate_count = 0;
	int half = array_size >> 1;
	for (int i = 0; i < array_size; i++)
	{
		intermediate_count += counts[i];
		if (intermediate_count >= half) // uh if we're halfway, just return lol
			return i;
	}

	assert(0);
	return 0;
}

FORCE_INLINE int median_slow(int* numbers, int count)
{
	std::sort(numbers, numbers + count); // could maybe replace this with something decent
	if (count % 2 != 0)
		return numbers[count >> 1];

	return (numbers[(count - 1) >> 1] + numbers[count >> 1]) >> 1;
}

FORCE_INLINE void calc_dist(uint64_t* cost2, int pos)
{
	for (int i = 0; i < max_pos; )
	{
		uint64_t a_pos = abs_lut[pos + max_pos];
		cost2[i] += cost2_lut[a_pos];
		pos--;
		i++;
	}
}

FORCE_INLINE int func(const char* file_name)
{
	char* cur, *end;
	get_file(file_name ? file_name : FILE_NAME, cur, end);

	static int input[max_pos];
	static uint64_t cost2[max_pos];
#if BENCHMARK
	if (BENCHMARK && !first)
		memset(input, 0, max_pos * sizeof(int));
	if (!first)
		memset(cost2, 0, max_pos * sizeof(uint64_t));

	result1 = 0;
	result2 = ~0;
#endif

	int input_count = 0;
	int total = 0;
	while (cur < end)
	{
		assert(input_count < max_pos);
		read_int(cur, input[input_count]);
		eat_non_number(cur, end);

		total += input[input_count];
		input_count++;
	}

	int pos1 = median(input, input_count);

	for (int j = 0; j < input_count; j++)
		calc_dist(cost2, input[j]);
	for (int i = 0; i < input_count; i++)
	{
		result1 += abs_lut[input[i] - pos1 + max_pos];
		if (result2 > cost2[i])
			result2 = cost2[i];
	}

	first = false;
	return result1;
}
