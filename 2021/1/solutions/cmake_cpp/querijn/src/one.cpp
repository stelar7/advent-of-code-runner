#include "common.hpp"

#include <iostream>
#include <fstream>

int part_one(const char* file)
{
	int number;

	std::ifstream input_file(file ? file : FILE_NAME);
	if (!input_file.is_open())
		return -1;

	int last_number = -1;
	int count = 0;
	input_file >> number;
	last_number = number;

	while (input_file >> number)
	{
		if (number > last_number)
			count++;
		last_number = number;
	}

	return count;
}
