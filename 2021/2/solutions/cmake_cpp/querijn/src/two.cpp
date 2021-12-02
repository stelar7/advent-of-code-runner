#include "common.hpp"

#include <iostream>
#include <fstream>

int values[2000];

int asdf(int i)
{
	return i % 3;
}

int two(const char* file)
{
	std::ifstream input_file(file ? file : FILE_NAME);
	if (!input_file.is_open())
		return -1;

	char text[32];
	int number;

	int x = 0;
	int y = 0;
	int aim = 0;

	while (true)
	{
		if (!(input_file >> text))
			break;

		input_file >> number;
		switch (text[0])
		{
		case 'd':
			aim += number;
			break;

		case 'u':
			aim -= number;
			break;

		case 'f':
			x += number;
			y += aim * number;
			break;
		}
	}

	return x * y;
}
