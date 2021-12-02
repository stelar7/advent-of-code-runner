#include <fstream>

#define FILE_NAME "input.txt"

int main(int argc, char* argv[])
{
	if (argc < 2)
	{
		printf("Missing input!");
		return -1;
	}

	const char* file = argv[1] ? argv[1] : FILE_NAME;
	std::ifstream input_file(file);
	if (!input_file.is_open())
	{
		printf("Missing file '%s'!", file);
		return -1;
	}

	char text[32];
	int number;

	int x1 = 0;
	int y1 = 0; // Y for part 1, aim for part two.
	int y2 = 0;

	while (true)
	{
		if (!(input_file >> text))
			break;

		input_file >> number;
		switch (text[0])
		{
		case 'd':
			y1 += number;
			break;

		case 'u':
			y1 -= number;
			break;

		case 'f':
			x1 += number;
			y2 += y1 * number;
			break;
		}
	}

	printf("%d\n%d\n", x1 * y1, x1 * y2);
	return 0;
}