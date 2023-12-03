#include "parse.hpp"
#include <unordered_map>
#include <vector>

// #define FILE_NAME "example.txt"
#define FILE_NAME "input.txt"

#define DATA_BUFFER_SIZE 23000
static int gears[DATA_BUFFER_SIZE];
void func(const char* file_name)
{
	char data[DATA_BUFFER_SIZE];
	char* cur = data;
	char* end;
	get_file(file_name ? file_name : FILE_NAME, cur, end, DATA_BUFFER_SIZE);
	result1 = 0;
	result2 = 0;


	// Get line width
	char* lineBegin = &data[0];
	char* lineEnd = strchr(cur, '\n');
	const size_t lineWidth = lineEnd - cur + 1;
	size_t lineCount = (end - cur) / (lineWidth);
	for (int line = 0; cur < end; line++)
	{
		if (line > 0)
			lineBegin = lineEnd + 1;
		lineEnd = cur + lineWidth-1;
		eat_non_number(cur, lineEnd);

		int gearLeft = -1;
		int gearRight = -1;

		int beginY = line == 0 ? 0 : -1;
		int endY = line == lineCount - 1 ? 0 : 1;

		while (cur < lineEnd)
		{
			int num = 0;
			char* numPos = cur;
			read_int(cur, num);
			int len = snprintf(nullptr, 0, "%d", num); // todo compare with numPos?

			bool found = false;
			bool gear = false;

			int beginX = numPos == lineBegin ? 0 : -1;
			int endX = numPos + len == lineEnd ? len - 1 : len;

			for (int y = beginY; y <= endY; ++y)
			{
				for (int x = beginX; x <= endX; ++x)
				{
					if (x == 0 && y == 0)
						continue;

					size_t offset = y * lineWidth + x;
					char t = numPos[offset];
					if (t != '.' && t != '\0' && t != '\n' && !(t >= '0' && t <= '9'))
					{
						found = true;

						if (t == '*')
						{
							auto e = (uint64_t)numPos - (uint64_t)data + (uint64_t)offset;
							auto& gear = gears[e];
							if (gear)
								result2 += gear * num;
							else
								gear = num;
						}
						break;
					}
				}

				if (found)
					break;
			}

			if (found)
				result1 += num;
			eat_non_number_line(cur, lineEnd);
		}
		cur = lineEnd + 1;
	}
}
