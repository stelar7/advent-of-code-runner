#include "common.hpp"

#include <fstream>

int one(const char* file)
{
	char text[32];
	int number;

	std::ifstream input_file(file ? file : FILE_NAME);
	if (!input_file.is_open())
		return -1;

	int x = 0;
	int y = 0;

	while (true)
	{
		if (!(input_file >> text))
			break;

		input_file >> number;
		switch (text[0])
		{
		case 'd':
			y += number;
			break;

		case 'u':
			y -= number;
			break;

		case 'f':
			x += number;
			break;
		}
	}

	return x * y;
}
