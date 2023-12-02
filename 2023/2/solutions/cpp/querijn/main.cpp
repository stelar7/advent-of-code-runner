#include "parse.hpp"

// #define FILE_NAME "example.txt"
#define FILE_NAME "input.txt"

void func(const char* file_name)
{
	static constexpr int redMax = 12;
	static constexpr int greenMax = 13;
	static constexpr int blueMax = 14;

#define DATA_BUFFER_SIZE 23000
	char data[DATA_BUFFER_SIZE];
	char* cur = data;
	char* end;
	get_file(file_name ? file_name : FILE_NAME, cur, end, DATA_BUFFER_SIZE);
	result1 = 0;
	result2 = 0;

	while (cur < end)
	{
		int gameId;
		int redCount = 0;
		int greenCount = 0;
		int blueCount = 0;

		int highestRedCount = 0;
		int highestGreenCount = 0;
		int highestBlueCount = 0;

		int count;
		bool valid = true;

		eat_non_number(cur, end); // Game
		read_int(cur, gameId); // 1
		eat_non_number(cur, end); // :

		while (*cur != '\n')
		{
			read_int(cur, count); // 1
			cur++; // skip space

			switch (*cur)
			{
			case 'r': // red
				redCount = count;
				if (redCount > highestRedCount)
					highestRedCount = redCount;
				break;

			case 'g': // green
				greenCount = count;
				if (greenCount > highestGreenCount)
					highestGreenCount = greenCount;
				break;

			case 'b': // blue
				blueCount = count;
				if (blueCount > highestBlueCount)
					highestBlueCount = blueCount;
				break;

			default:
				assert(false);
			}

			eat_non_number_line(cur, end);
			if (redCount > redMax || greenCount > greenMax || blueCount > blueMax)
			{
				valid = false;
			}
		}
		cur++;

		if (valid)
			result1 += gameId;

		int power = highestRedCount * highestGreenCount * highestBlueCount;
		result2 += power;
	}
}
