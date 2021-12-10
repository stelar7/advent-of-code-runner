import sys
import json

input = sys.stdin.readlines()

lowpoints = []
basins = []

risk = 0

def appendNeighs(holder, x, y):
	if y < 0 or y >= len(input):
		return
	if x < 0 or x >= len(input[y]):
		return

	value = input[y][x]
	if value == "9" or value == "\n":
		return
	if json.dumps([x,y]) in json.dumps(holder):
		return
	holder.append([x, y])
	appendNeighs(holder, x+1, y)
	appendNeighs(holder, x-1, y)
	appendNeighs(holder, x, y+1)
	appendNeighs(holder, x, y-1)

lowcoords = []

for line_index, line in enumerate(input):
	line = line.strip('\n')
	y = line_index
	for num_index, num in enumerate(line):
		if num == "\n":
			continue
		x = num_index
		if line_index == 0 and num_index == 0:
			if num < line[num_index + 1] and num < input[line_index +1][num_index]:
				lowpoints.append(num)
				lowcoords.append([x,y])
		elif num_index == 0:
			if num < line[num_index + 1] and num < input[line_index-1][num_index] and num < input[line_index +1][num_index]:
				lowpoints.append(num)
				lowcoords.append([x,y])
		elif num_index == (len(line) - 1): #and line_index != len(input) -1:
			if num < line[num_index- 1] and num < input[line_index +1][num_index] and num < input[line_index - 1][num_index]:
				lowpoints.append(num)
				lowcoords.append([x,y])
		elif line_index == len(input)-1 and num_index != len(line) -1:
			if num < line[num_index - 1] and num < line[num_index +1] and num < input[line_index -1][num_index]:
				lowpoints.append(num)
				lowcoords.append([x,y])
		elif num_index == len(line) - 1 and line_index == len(input) -1:
			if num < line[num_index -1] and num < input[line_index -1][num_index]:
				lowpoints.append(num)
				lowcoords.append([x,y])
		elif num_index == len(line) -1 and line_index == 0:
			if num < line[num_index +1] and num < input[line_index +1][num_index]:
				lowpoints.append(num)
				lowcoords.append([x,y])
		else:
			if num < line[num_index + 1] and num < input[line_index+1][num_index] and num < line[num_index-1] and num < input[line_index - 1][num_index]:
				lowpoints.append(num)
				lowcoords.append([x,y])
		#print(len(lowpoints))

for begin in lowcoords:
	beginX = begin[0]
	beginY = begin[1]
	parts = []
	appendNeighs(parts, beginX, beginY)
	basins.append(len(parts))

three = sorted([x for x in basins], reverse=True)[:3]

total = 1
for basin in three:
	total = total * basin

for low in lowpoints:
	risk += (int(low))

print(risk + len(lowpoints))
print(total)
