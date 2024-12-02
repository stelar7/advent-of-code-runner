import sys

numbers = [tuple(map(int, line.strip().split())) for line in sys.stdin] # # [(2, 3), (1, 5), ...]

left, right = zip(*numbers) # (2, 1, ...), (3, 5, ...)

left, right = sorted(left), sorted(right)

# Part 1: Calculate total distance
total_distance = sum(abs(l - r) for l, r in zip(left, right))

# Part 2: Calculate matching score
score = 0
for num in left:
    matching_count = right.count(num)
    score += num * matching_count

print(total_distance)
print(score)
