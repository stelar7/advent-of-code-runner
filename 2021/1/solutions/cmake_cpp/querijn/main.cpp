#include <fstream>

#define FILE_NAME "input.txt"

#define DO_ONE(Variable) do { \
	if (Variable > last_number) \
		count1++; \
	last_number = Variable; \
} while(0)

int main(int argc, char* argv[])
{
	const char* file = argv[1] ? argv[1] : FILE_NAME;
	std::ifstream input_file(file);

	int number;
	int count1 = 0;
	int count2 = 0;
	int last_number = -1;

	int old1; input_file >> old1; last_number = old1;
	int old2; input_file >> old2; DO_ONE(old2);
	int old3; input_file >> old3; DO_ONE(old3);

	while (input_file >> number)
	{
		DO_ONE(number);

		if (old1 < number)
			count2++;
		old1 = old2;
		old2 = old3;
		old3 = number;
	}

	printf("%d\n%d\n", count1, count2);
	return 0;
}