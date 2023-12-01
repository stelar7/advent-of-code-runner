#include "parse.hpp"

constexpr uint32_t FourChars(char a, char b, char c, char d)
{
	return a | b << 8 | c << 16 | d << 24;
}

#define FOUR_CHARS_CASE(a, b) case FourChars(a[0], a[1], a[2], a[3]): return b;
#define FIVE_CHARS_CASE(a, t, b) case FourChars(a[0], a[1], a[2], a[3]): { if (a[4] == cur[4]) { return b; } return -1; }
#define THREE_CHARS_CASE(a, b) \
	case FourChars(a[0], a[1], a[2], 0): return b;\
	case FourChars(a[0], a[1], a[2], '\r'): return b;\
	case FourChars(a[0], a[1], a[2], '\n'): return b;\
	case FourChars(a[0], a[1], a[2], '0'): return b;\
	case FourChars(a[0], a[1], a[2], '1'): return b;\
	case FourChars(a[0], a[1], a[2], '2'): return b;\
	case FourChars(a[0], a[1], a[2], '3'): return b;\
	case FourChars(a[0], a[1], a[2], '4'): return b;\
	case FourChars(a[0], a[1], a[2], '5'): return b;\
	case FourChars(a[0], a[1], a[2], '6'): return b;\
	case FourChars(a[0], a[1], a[2], '7'): return b;\
	case FourChars(a[0], a[1], a[2], '8'): return b;\
	case FourChars(a[0], a[1], a[2], '9'): return b;\
	case FourChars(a[0], a[1], a[2], 'a'): return b;\
	case FourChars(a[0], a[1], a[2], 'b'): return b;\
	case FourChars(a[0], a[1], a[2], 'c'): return b;\
	case FourChars(a[0], a[1], a[2], 'd'): return b;\
	case FourChars(a[0], a[1], a[2], 'e'): return b;\
	case FourChars(a[0], a[1], a[2], 'f'): return b;\
	case FourChars(a[0], a[1], a[2], 'g'): return b;\
	case FourChars(a[0], a[1], a[2], 'h'): return b;\
	case FourChars(a[0], a[1], a[2], 'i'): return b;\
	case FourChars(a[0], a[1], a[2], 'j'): return b;\
	case FourChars(a[0], a[1], a[2], 'k'): return b;\
	case FourChars(a[0], a[1], a[2], 'l'): return b;\
	case FourChars(a[0], a[1], a[2], 'm'): return b;\
	case FourChars(a[0], a[1], a[2], 'n'): return b;\
	case FourChars(a[0], a[1], a[2], 'o'): return b;\
	case FourChars(a[0], a[1], a[2], 'p'): return b;\
	case FourChars(a[0], a[1], a[2], 'q'): return b;\
	case FourChars(a[0], a[1], a[2], 'r'): return b;\
	case FourChars(a[0], a[1], a[2], 's'): return b;\
	case FourChars(a[0], a[1], a[2], 't'): return b;\
	case FourChars(a[0], a[1], a[2], 'u'): return b;\
	case FourChars(a[0], a[1], a[2], 'v'): return b;\
	case FourChars(a[0], a[1], a[2], 'w'): return b;\
	case FourChars(a[0], a[1], a[2], 'x'): return b;\
	case FourChars(a[0], a[1], a[2], 'y'): return b;\
	case FourChars(a[0], a[1], a[2], 'z'): return b;

// #define FILE_NAME "example.txt"
#define FILE_NAME "input.txt"

FORCE_INLINE int GetNum(char*& cur)
{
	uint32_t test = *reinterpret_cast<const uint32_t*>(cur);
	switch (test)
	{
	THREE_CHARS_CASE("one", 1);
	THREE_CHARS_CASE("two", 2);
	FIVE_CHARS_CASE("three", cur, 3);
	FOUR_CHARS_CASE("four", 4);
	FOUR_CHARS_CASE("five", 5);
	THREE_CHARS_CASE("six", 6);
	FIVE_CHARS_CASE("seven", cur, 7);
	FIVE_CHARS_CASE("eight", cur, 8);
	FOUR_CHARS_CASE("nine", 9);
	FOUR_CHARS_CASE("zero", 0);
	}

	return -1;
}

void func(const char* file_name)
{
	char* cur, * end;
	get_file(file_name ? file_name : FILE_NAME, cur, end);
	result1 = 0;
	result2 = 0;
	int line = 1;

	while (cur < end)
	{
		int first = -1;
		int last = -1;
		int first2 = -1;
		int last2 = -1;

		while (true)
		{
			if (*cur >= '0' && *cur <= '9')
			{
				int n = *cur - '0';
				if (first2 < 0)
					first2 = n;
				last2 = n;

				if (first < 0)
					first = n;
				last = n;
			}
			else if (*cur >= 'a' && *cur <= 'z')
			{
				int n = GetNum(cur);
				if (n >= 0)
				{
					if (first2 < 0)
						first2 = n;
					last2 = n;
				}
			}

			cur++;
			if (*cur == '\n' || *cur == 0)
				break;
		}

		cur++;

		if (first < 0) first = 0;
		if (last < 0) last = 0;
		int n1 = first * 10 + last;
		result1 += n1;

		assert(first2 >= 0 && last2 >= 0);
		int n2 = first2 * 10 + last2;

		result2 += n2;
		line++;
	}
}
