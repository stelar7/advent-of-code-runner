import sys

lines = sys.stdin.readlines()

p1 = False

for line in lines:
	line = line.rstrip()
	past_four = []
	past_fourteen = []

	for index, char in enumerate(line):
		past_four.append(char)
		past_fourteen.append(char)

		while len(past_four) > 4:
			del past_four[0]
		while len(past_fourteen) > 14:
			del past_fourteen[0]

		if len(past_four) == 4 and not p1:
			if len(past_four) > len(set(past_four)):
				continue
			else:
				print(index+1)
				p1 = True

		if len(past_fourteen) == 14:
			if len(past_fourteen) > len(set(past_fourteen)):
				continue
			else:
				print(index+1)
				break
