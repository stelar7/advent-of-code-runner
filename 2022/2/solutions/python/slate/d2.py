import sys

input = sys.stdin.readlines()

score = 0 
new_score = 0

for line in input:
	opponent_move = line.split(' ')[0]
	your_move = line.split(' ')[1].rstrip()

	# X = Rock 
	# Y = Paper
	# Z = Scissors

	# A = Rock
	# B = Paper
	# C = Scissors

	if your_move == "X":
		score += 1
		if opponent_move == "C":
			score += 6
			new_score += 2
		if opponent_move == "A":
			score += 3
			new_score += 3
		if opponent_move == "B":
			new_score += 1

	if your_move == "Y":
		score += 2
		new_score += 3
		if opponent_move == "A":
			score += 6
			new_score += 1
		if opponent_move == "B":
			score += 3
			new_score += 2
		if opponent_move == "C":
			new_score += 3

	if your_move == "Z":
		score += 3
		new_score += 6
		if opponent_move == "B":
			score += 6
			new_score += 3
		if opponent_move == "C":
			score += 3
			new_score += 1
		if opponent_move == "A":
			new_score += 2

print(score)
print(new_score)
