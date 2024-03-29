import sys

input = sys.stdin.readlines()

cals = []
total_cals = []

for line in input:
	sum_cals = 0
	if line == "\n":
		for cal in cals:
			sum_cals += int(cal)
		total_cals.append(sum_cals)
		cals.clear()
	elif line == input[-1]:
		total_cals.append(int(line.rstrip()))
	else:
		cals.append(line.rstrip())

print(max(total_cals))

top_three = []

for i in range(3):
	top_three.append(max(total_cals))
	total_cals.remove(max(total_cals))

total = sum(top_three)

print(str(total))
