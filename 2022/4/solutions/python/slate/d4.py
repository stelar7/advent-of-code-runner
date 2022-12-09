import sys

input = sys.stdin.readlines()

part1_count = 0
part2_count = 0

for line in input:
	pairs = line.rstrip().split(',')
	p1_n1 = pairs[0].split('-')[0] # pair 1 num 1
	p1_n2 = pairs[0].split('-')[1] # pair 1 num 2
	p2_n1 = pairs[1].split('-')[0] # pair 2 num 1
	p2_n2 = pairs[1].split('-')[1] # pair 2 num 2

	p1s = [] # pair 1s
	p2s = [] # pair 2s

	contained = True
	contained2 = True
	overlap = False

	for i in range(int(p1_n1), int(p1_n2)+1):
		p1s.append(i)

	for j in range(int(p2_n1), int(p2_n2)+1):
		p2s.append(j)

	for k in p1s:
		if k in p2s:
			overlap = True
			continue
		else:
			contained = False

	for l in p2s:
		if l in p1s:
			overlap = True
			continue
		else:
			contained2 = False

	if contained or contained2:
		part1_count += 1

	if overlap:
		part2_count += 1

print(part1_count)
print(part2_count)
