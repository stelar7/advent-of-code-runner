#include <fstream>
#include <cassert>
#include <cstdio>
#include <string>
#include <sstream>
#include <vector>
#include <algorithm>
#include <cstring>

#define BENCHMARK false
// #define BENCHMARK !_DEBUG || defined(_WIN32)

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

// #define FILE_NAME "example.txt"
#define FILE_NAME "input.txt"

#define WIDTH 1000
int board1[WIDTH * WIDTH];
int board2[WIDTH * WIDTH];

#define DATA_BUFFER_SIZE 20000
char data_buffer[DATA_BUFFER_SIZE];
int count1 = 0;
int count2 = 0;

#if BENCHMARK
bool first_run = true;
#endif

FORCE_INLINE void get_file(const char* file_name, char*& data, char*& end_data)
{
	data = data_buffer;

	std::ifstream input(file_name, std::ios::binary);
	if (!input.is_open())
		return;

	input.seekg(0, input.end);
	size_t len = input.tellg();
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

FORCE_INLINE void debug_output()
{
#if _DEBUG && WIDTH <= 10
	for (int y = 0; y < WIDTH; y++)
	{
		for (int x = 0; x < WIDTH; x++)
		{
			printf("%d ", board1[x + y * WIDTH]);
		}

		printf("\n");
	}
	printf("\n");
#endif
}

FORCE_INLINE void draw_horizontal(int* board, int& count, int& v_x1, int& v_y1, int& v_x2, int& v_y2)
{
	int* entry = &board[v_y1 * WIDTH + v_x1];
	int* end = &board[v_y2 * WIDTH + v_x1];
	for (; entry <= end; entry += WIDTH)
	{
		(*entry)++;
		if (*entry == 2)
			count++;
	}
}

FORCE_INLINE void draw_vertical(int* board, int& count, int v_x1, int v_y1, int v_x2, int v_y2)
{
	int* entry = &board[v_y1 * WIDTH + v_x1];
	int* end = &board[v_y2 * WIDTH + v_x2];
	for (; entry <= end; entry++)
	{
		(*entry)++;
		if (*entry == 2)
			count++;
	}
}

FORCE_INLINE void draw_board(int v_x1, int v_y1, int v_x2, int v_y2)
{
	if (v_x1 == v_x2)
	{
		if (v_y1 > v_y2)
			std::swap(v_y1, v_y2);

		draw_horizontal(board1, count1, v_x1, v_y1, v_x2, v_y2);
		draw_horizontal(board2, count2, v_x1, v_y1, v_x2, v_y2);
	}
	else if (v_y1 == v_y2)
	{
		if (v_x1 > v_x2)
			std::swap(v_x1, v_x2);

		draw_vertical(board1, count1, v_x1, v_y1, v_x2, v_y2);
		draw_vertical(board2, count2, v_x1, v_y1, v_x2, v_y2);
	}
	else // diagonally
	{
		ASS(v_x1 != v_x2);
		ASS(v_y1 != v_y2);

		int dir;
		if (v_x1 < v_x2)
			dir = 1;
		else
			dir = -1;

		if (v_y1 < v_y2)
			dir += 1 * WIDTH;
		else
			dir += -1 * WIDTH;

		int* pos = &board2[v_x1 + v_y1 * WIDTH];
		int* pos_end = &board2[v_x2 + v_y2 * WIDTH];
		bool should_break = false;
		do
		{
			(*pos)++;
			if ((*pos) == 2)
				count2++;

			pos += dir;
			if (should_break)
				break;
			if (pos == pos_end)
				should_break = true; // Break after next loop
		} while (true);
	}

	debug_output();
}

FORCE_INLINE int func(const char* file_name)
{
	char* cur, *end;
	get_file(file_name ? file_name : FILE_NAME, cur, end);

	// This needs to run during benchmarks because the data has been used
#if BENCHMARK
	if (first_run)
	{
		count1 = 0;
		count2 = 0;

		memset(board1, 0, sizeof(int) * WIDTH * WIDTH);
		memset(board2, 0, sizeof(int) * WIDTH * WIDTH);
		first_run = false;
	}
#endif

	int v_x1, v_x2, v_y1, v_y2;
	while (cur != end)
	{
		read_int(cur, v_x1);
		cur++;
		read_int(cur, v_y1);

		cur += 4; // strlen(" -> ");

		read_int(cur, v_x2);
		cur++;
		read_int(cur, v_y2);

		while (*cur == '\r' || *cur == '\n')
			cur++;
		draw_board(v_x1, v_y1, v_x2, v_y2);
	}

	return count2;
}

#include <chrono>
#if !BENCHMARK

int main(int argc, char* argv[])
{
	std::ios::sync_with_stdio(false);

	func(argv[1]);

	printf("%d\n%d\n", count1, count2);
	return 0;
}

#else

int main(int argc, char* argv[])
{
	std::ios::sync_with_stdio(false);

	constexpr int runs = 1500;
	auto start = std::chrono::steady_clock::now();
	for (int i = 0; i < runs; i++)
		func(argv[1]);
	auto end = std::chrono::steady_clock::now();
	double time_taken = std::chrono::duration<double>(end - start).count();
	printf("Total: %3.2f sec\nAvg: %5.3f ms per run", time_taken, (time_taken / (double)runs) * 1000.0);
}

#endif
