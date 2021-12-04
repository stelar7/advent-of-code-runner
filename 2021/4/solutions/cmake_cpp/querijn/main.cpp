#include <fstream>
#include <cassert>
#include <string>
#include <sstream>
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

#define FILE_NAME "example.txt"
// #define FILE_NAME "input.txt"

template<int x, int y>
static constexpr inline int get_index()
{
	int&& xo = x;
	int&& yo = y;
	if (xo < 0)
		xo += 5;
	if (yo < 0)
		yo += 5;
	if (xo >= 5)
		xo -= 5;
	if (yo >= 5)
		yo -= 5;

	return yo * 5 + xo;
}

struct Board
{
	int field[5 * 5];
	int vert[5 * 5];
	bool is_done = false;

	Board()
	{
		memset(field, 0, 5 * 5 * sizeof(int));
	}

	template<int x, int y>
	constexpr bool do_check_single() const
	{
		if (field[get_index<x, y>()] == -1)
		{
			if (field[get_index<x + 1, y>()] == -1 &&
				field[get_index<x + 2, y>()] == -1 &&
				field[get_index<x + 3, y>()] == -1 &&
				field[get_index<x + 4, y>()] == -1)
				return true;

			if (field[get_index<x, y + 1>()] == -1 &&
				field[get_index<x, y + 2>()] == -1 &&
				field[get_index<x, y + 3>()] == -1 &&
				field[get_index<x, y + 4>()] == -1)
				return true;
		}

		return false;
	}

	bool do_check() const
	{
		if (do_check_single<0, 0>())
			return true;
		if (do_check_single<1, 1>())
			return true;
		if (do_check_single<2, 2>())
			return true;
		if (do_check_single<3, 3>())
			return true;
		if (do_check_single<4, 4>())
			return true;

		return false;
	}

	bool mark(int m, bool check)
	{
		for (int i = 0; i < 5 * 5; i++)
		{
			if (field[i] != m)
				continue;

			field[i] = -1;
		}

		/*printf("%d %d %d %d %d\n",	field[get_index<0, 0>()],
									field[get_index<1, 0>()],
									field[get_index<2, 0>()],
									field[get_index<3, 0>()],
									field[get_index<4, 0>()]);

		printf("%d %d %d %d %d\n",	field[get_index<0, 1>()],
									field[get_index<1, 1>()],
									field[get_index<2, 1>()],
									field[get_index<3, 1>()],
									field[get_index<4, 1>()]);

		printf("%d %d %d %d %d\n",	field[get_index<0, 2>()],
									field[get_index<1, 2>()],
									field[get_index<2, 2>()],
									field[get_index<3, 2>()],
									field[get_index<4, 2>()]);

		printf("%d %d %d %d %d\n",	field[get_index<0, 3>()],
									field[get_index<1, 3>()],
									field[get_index<2, 3>()],
									field[get_index<3, 3>()],
									field[get_index<4, 3>()]);

		printf("%d %d %d %d %d\n\n",	field[get_index<0, 4>()],
									field[get_index<1, 4>()],
									field[get_index<2, 4>()],
									field[get_index<3, 4>()],
									field[get_index<4, 4>()]);*/

		if (!check)
			return false;
		return do_check();
	}

	inline int count() const
	{
		int total = 0;
		for (int i = 0; i < 5*5; i++)
			if (field[i] != -1)
				total += field[i];
		return total;
	}
};

std::vector<Board> boards;
std::vector<int> picks;

FORCE_INLINE int func(const char* file_name)
{
	const char* file = file_name ? file_name : FILE_NAME;
	std::ifstream input_file(file);
	if (!input_file.is_open())
	{
		printf("Missing file '%s'!", file);
		return -1;
	}

	// picks
	int number; char div;
	std::string line;
	std::getline(input_file, line);
	std::stringstream linestream(line);
	while (linestream >> number)
	{
		picks.push_back(number);
		linestream >> div;
	}

	// boards
	Board board;
	int i = 0;
	while (input_file >> board.field[i])
	{
		i++;
		if (i == 5 * 5)
		{
			boards.push_back(board);
			i = 0;
		}
	}

	bool has_first = false;
	int answer1 = 0;
	int last_pick1 = 0;

	int answer2 = 0;
	int last_pick2 = 0;

	int it = 0;
	i = 0;
	for (; i < 4; i++)
		for (auto& board : boards)
			board.mark(picks[i], false);

	int* cur_pick = &picks[i];
	int* picks_end = &picks.back() + 1;
	for (; cur_pick != picks_end; cur_pick++)
	{
		for (auto& board : boards)
		{
			if (board.is_done)
				continue;
			if (board.mark(*cur_pick, true) == false)
				continue;
			
			answer2 = board.count();
			last_pick2 = *cur_pick;
			board.is_done = true;

			if (has_first == false)
			{
				answer1 = answer2;
				last_pick1 = *cur_pick;
				has_first = true;
			}
		}

		for (auto& board : boards)
			if (board.is_done == false)
				goto next;

		printf("%d\n%d\n", answer1 * last_pick1, answer2 * last_pick2);
		return answer2 * last_pick2;

	next:
		continue;
	}

	return -1;
}

#include <chrono>

#if 1 // _DEBUG || !defined(_WIN32)

int main(int argc, char* argv[]) { func(argv[1]); }

#else

int main(int argc, char* argv[])
{
	constexpr int runs = 200;
	auto start = std::chrono::steady_clock::now();
	for (int i = 0; i < runs; i++)
		func(argv[1]);
	auto end = std::chrono::steady_clock::now();
	system("cls");
	double time_taken = std::chrono::duration<double>(end - start).count();
	printf("Total: %3.5f sec\nAvg: %3.5f sec per run", time_taken, time_taken / (double)runs);
}

#endif
