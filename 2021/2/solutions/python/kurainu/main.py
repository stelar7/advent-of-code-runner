import sys
Lines = sys.stdin.readlines()

horizontal_pos = 0
depth = 0

for line in Lines:
  direction, amount = line.split(' ')
  if direction == "forward":
    horizontal_pos += int(amount)
  elif direction == "down":
    depth += int(amount)
  elif direction == "up":
    depth -= int(amount)

print(horizontal_pos*depth)

horizontal_pos = 0
depth = 0
aim = 0

for line in Lines:
  direction, amount = line.split(' ')
  if direction == "forward":
    horizontal_pos += int(amount)
    depth += int(amount) * aim
  elif direction == "down":
    aim += int(amount)
  elif direction == "up":
    aim -= int(amount)

print(horizontal_pos*depth)
