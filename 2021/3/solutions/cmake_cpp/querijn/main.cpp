#include <fstream>
#include <cassert>
#include <string>
#include <vector>
#include <algorithm>
#include <cstring>

#if defined(_MSC_VER)
#define FORCE_INLINE __forceinline
#elif defined(__GNUC__)
#define FORCE_INLINE inline __attribute__((always_inline))
#elif (__STDC_VERSION__ >= 199901L)
#define FORCE_INLINE INLINE
#else
#define FORCE_INLINE INLINE
#endif

// #define FILE_NAME "example.txt"
#define FILE_NAME "input.txt"
// #define FILE_NAME "test.txt"
constexpr int max_line_length = 12;

template<typename Iterator>
FORCE_INLINE void count_set(const std::vector<std::string>& set, int& count_0, int& count_1, int i, Iterator min, Iterator max, int length)
{
	count_0 = 0;
	count_1 = 0;

	for (auto j = min; j != max; j++)
	{
		//for (int i = 0; i < length; i++)
		{
			auto&& input = (*j);
			if (input[i] == '0')
				count_0++;
			else
				count_1++;
		}
	}
}

FORCE_INLINE int func(const char* file_name)
{
	const char* file = file_name ? file_name : FILE_NAME;
	std::ifstream input_file(file);
	if (!input_file.is_open())
	{
		printf("Missing file '%s'!", file);
		return -1;
	}

	int line_length = 0;
	std::vector<std::string> oxygen;
	int gamma = 0;
	int eps = 0;

	// Part 1
	{
		int count_1[max_line_length];
		int count_0[max_line_length];

		std::string input;
		memset(count_0, 0, sizeof(int) * max_line_length);
		memset(count_1, 0, sizeof(int) * max_line_length);

		while (input_file >> input)
		{
			if (line_length == 0) line_length = input.length();

			for (int i = 0; i < line_length; i++)
				if (input[i] == '0')
					count_0[i]++;
				else
					count_1[i]++;

			oxygen.push_back(input);
		}

		for (int i = 0; i < line_length; i++)
		{
			if (count_1[i] > count_0[i])
				gamma |= 0b1 << (line_length - 1 - i);
			if (count_0[i] > count_1[i])
				eps |= 0b1 << (line_length - 1 - i);
		}
	}

	std::sort(oxygen.begin(), oxygen.end());
	int oxy_or = 0;
	const std::string* oxy_res = nullptr;

	std::vector<std::string> co2 = oxygen;
	int co2_and = 0;
	const std::string* co2_res = nullptr;

	int count_0;
	int count_1;
	auto min_index = oxygen.begin();
	auto max_index = oxygen.end();

	for (int i = 0; i < line_length; i++)
	{
#if _DEBUG
		std::vector<std::string> debug(min_index, max_index);
#endif

		count_set(oxygen, count_0, count_1, i, min_index, max_index, line_length);

		bool is_1 = count_1 >= count_0;
		if (count_1 == count_0)
			oxy_or |= 0b1 << (line_length - 1 - i);

		if (oxy_res == nullptr)
		{
			int count = 0;

			// Find begin
			auto elem = min_index;
			for (; elem != max_index; elem++)
			{
				if ((is_1 == ((*elem)[i] == '1')) == false)
					continue;

				count++;
				if (count == 2)
					break;
				else
					min_index = elem;
			}

			// find end
			elem = min_index;
			for (; elem != max_index; elem++)
			{
				if (is_1 == ((*elem)[i] == '1'))
					continue;

				max_index = elem;
				break;
			}

			if (count == 1)
				oxy_res = &(*min_index);
		}
	}

	min_index = co2.begin();
	max_index = co2.end();

	for (int i = 0; i < line_length; i++)
	{
		count_set(co2, count_0, count_1, i, min_index, max_index, line_length);

		bool is_1 = count_1 >= count_0;
		if (count_1 == count_0)
			co2_and |= 0b1 << (line_length - 1 - i);

		if (co2_res == nullptr)
		{
			int count = 0;

			// Find begin
			auto elem = min_index;
			for (; elem != max_index; elem++)
			{
				if ((is_1 != ((*elem)[i] == '1')) == false)
					continue;

				count++;
				if (count == 2)
					break;
				else
					min_index = elem;
			}

			// find end
			elem = min_index;
			for (; elem != max_index; elem++)
			{
				if (is_1 != ((*elem)[i] == '1'))
					continue;

				max_index = elem;
				break;
			}

			if (count == 1)
				co2_res = &(*min_index);
		}
	}

	int oxy_value = strtol(oxy_res->c_str(), nullptr, 2) | oxy_or;
	int co2_value = strtol(co2_res->c_str(), nullptr, 2) & ~co2_and;
	printf("%d\n%d\n", gamma * eps, oxy_value * co2_value);
	return 0;
}

#include <chrono>

#if 1 // _DEBUG || !defined(_WIN32)

int main(int argc, char* argv[]) { func(argv[1]); }

#else

int main(int argc, char* argv[])
{
	constexpr int runs = 1000;
	auto start = std::chrono::steady_clock::now();
	for (int i = 0; i < runs; i++)
		func(argv[1]);
	auto end = std::chrono::steady_clock::now();
	system("cls");
	double time_taken = std::chrono::duration<double>(end - start).count();
	printf("Total: %3.5f sec\nAvg: %3.5f sec per run", time_taken, time_taken / (double)runs);
}

#endif
