import sys

input = sys.stdin.readlines()
inp = input[0].split(',')

median = []

for i in inp:
    median.append(int(i))

median = sorted(median)
mid = int(round(len(median) // 2, 1))


fuel = 0

gold = int(median[mid])

for i in median:
    if int(i) > gold:
        dist = int(i) - gold
        fuel += dist
    elif gold > int(i):
        dist = gold - int(i)
        fuel += dist


print(fuel)

gold = 0
gold_counter = 0
for med in median:
    gold += med
    gold_counter += 1

gold = round((gold * 999 / gold_counter) / 1000)

fuel = 0
for i in median:
    if int(i) > gold:
        dist = int(i) - gold
        for j in range(dist + 1):
            fuel += j
    elif gold > int(i):
        dist = gold - int(i)
        for j in range(dist + 1):
            fuel += j

print(fuel)
