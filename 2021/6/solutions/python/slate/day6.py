import sys

input = sys.stdin.readlines()

inp = input[0].split(',')

fish = [0] * 9

for i, val in enumerate(inp):
	intval = int(val)
	fish[intval]+=1

counter = 0
counter2 = 0

for j in range(80):
	tmp = fish[0]
	for ii in range(len(fish) - 1):
		fish[ii] = fish[ii+1]
	fish[6] += tmp
	fish[8] = tmp

for fi in fish:
	counter += fi

print(counter)

fish = [0] * 9

for i, val in enumerate(inp):
	intval = int(val)
	fish[intval] += 1

for j in range(256):
	tmp = fish[0]
	for ii in range(len(fish) - 1):
		fish[ii] = fish[ii+1]
	fish[6] += tmp
	fish[8] = tmp

for fi in fish:
	counter2 += fi

print(counter2)
