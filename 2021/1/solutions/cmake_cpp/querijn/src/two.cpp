#include "common.hpp"

#include <fstream>

int part_two(const char* file)
{
	int number, count = 0;

	std::ifstream input_file(file ? file : FILE_NAME);
	if (!input_file.is_open())
		return -1;

	int old1; input_file >> old1;
	int old2; input_file >> old2;
	int old3; input_file >> old3;

	while (input_file >> number)
	{
		if (old1 < number)
			count++;
		old1 = old2;
		old2 = old3;
		old3 = number;
	}

	return count;
}
