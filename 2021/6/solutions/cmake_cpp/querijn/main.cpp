#define BENCHMARK 0

#pragma region COMMON
#include <fstream>
#include <cassert>
#include <cstdint>

#if defined(_DEBUG)
#define ASS(a) assert(a)
#else
#define ASS(a) ((void)0)
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
uint64_t result1 = 0;
uint64_t result2 = 0;

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
	int c;
	n = *input - '0';
	input++;
	while (*input >= '0' && *input <= '9')
	{
		n = 10 * n + *input - '0';
		input++;
	}
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

	printf("%llu\n", result1);
	printf("%llu\n", result2);
}

#else

int main(int argc, char* argv[])
{
	func(argv[1]);

	printf("%llu\n", result1);
	printf("%llu\n", result2);
	return 0;
}

#endif
#pragma endregion

// #define FILE_NAME "example.txt"
#define FILE_NAME "input.txt"

FORCE_INLINE void simulate_day(uint64_t* day_increment, uint64_t& cur_day, uint64_t& day_count, uint64_t& tomorrow_inc, size_t& total)
{
	uint64_t increment = day_increment[cur_day];
	day_increment[cur_day] += tomorrow_inc;
	tomorrow_inc = 0;

	if (increment)
	{
		total += increment;

		// Today's fish move to 7 days from now
		day_increment[cur_day] -= increment;
		int in_7_days = (cur_day + 7) % 8;
		day_increment[in_7_days] += increment;

		// New fish will duplicate on a week from tomorrow
		int in_8_days = (cur_day + 9) % 8;
		tomorrow_inc += increment;
	}

	cur_day++;
	if (cur_day == 8)
		cur_day = 0;
	day_count++;
}

FORCE_INLINE int func(const char* file_name)
{
	char* cur, *end;
	get_file(file_name ? file_name : FILE_NAME, cur, end);

	uint64_t day_increment[8];
	memset(day_increment, 0, 8 * sizeof(uint64_t));

	uint64_t cur_day = 0;
	uint64_t day_count = 0;
	uint64_t tomorrow_inc = 0;

	uint64_t total = 0;

	bool first = 1;
	while (cur < end)
	{
		int delay;
		read_int(cur, delay);
		day_increment[delay % 8]++;
		total++;

		eat_non_number(cur, end);
	}

	while (day_count != 80)
		simulate_day(day_increment, cur_day, day_count, tomorrow_inc, total);
	result1 = total;

	while (day_count != 256)
		simulate_day(day_increment, cur_day, day_count, tomorrow_inc, total);
	result2 = total;

	return result1;
}