import sys
Lines = list(map(int, sys.stdin.readlines()))

Counter = 0
linebefore = 0

for line in Lines:
    if int(line) > int(linebefore):
        Counter += 1
    linebefore = line

print(Counter - 1)

Counter = 0
previous_sum = 0

for i in range(len(Lines)):
    measure_window = Lines[i:i+3]
    if len(measure_window) != 3:
        break

    if sum(measure_window) > previous_sum:
        Counter += 1
    previous_sum = sum(measure_window)

print(Counter - 1)
