#include "common.hpp"

#include <iostream>
#include <fstream>

int TWO(const char* file)
{
	int number, count = 0;

	std::ifstream input_file(file ? file : FILE_NAME);
	if (!input_file.is_open())
		return -1;

	int window1 = 0, count1 = 0;
	int window2 = 0, count2 = 0;
	int window3 = 0, count3 = 0;

	input_file >> number;
	window1 += number;
	count1++;

	input_file >> number;
	window1 += number;
	window2 += number;
	count1++;
	count2++;

	while (input_file >> number)
	{
		if (count1 != 3) { window1 += number; count1++; }
		if (count2 != 3) { window2 += number; count2++; }
		if (count3 != 3) { window3 += number; count3++; }

		if (count1 == 3 && count2 == 3)
		{
			if (window2 > window1)
				count++;
			count1 = 1;
			window1 = number;
		}

		if (count2 == 3 && count3 == 3)
		{
			if (window3 > window2)
				count++;
			count2 = 1;
			window2 = number;
		}

		if (count1 == 3 && count3 == 3)
		{
			if (window1 > window3)
				count++;
			count3 = 1;
			window3 = number;
		}
	}

	return count;
}
