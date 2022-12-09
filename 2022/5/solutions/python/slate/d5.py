import sys

lines = sys.stdin.readlines()

one = ["B", "P", "N", "Q", "H", "D", "R", "T"]
two = ["W", "G", "B", "J", "T", "V"]
three = ["N", "R", "H", "D", "S", "V", "M", "Q"]
four = ["P", "Z", "N", "M", "C"]
five = ["D", "Z", "B"]
six = ["V", "C", "W", "Z"]
seven = ["G", "Z", "N", "C", "V", "Q", "L", "S"]
eight = ["L", "G", "J", "M", "D", "N", "V"]
nine = ["T", "P", "M", "F", "Z", "C", "G"]

p1_one = ["B", "P", "N", "Q", "H", "D", "R", "T"]
p1_two = ["W", "G", "B", "J", "T", "V"]
p1_three = ["N", "R", "H", "D", "S", "V", "M", "Q"]
p1_four = ["P", "Z", "N", "M", "C"]
p1_five = ["D", "Z", "B"]
p1_six = ["V", "C", "W", "Z"]
p1_seven = ["G", "Z", "N", "C", "V", "Q", "L", "S"]
p1_eight = ["L", "G", "J", "M", "D", "N", "V"]
p1_nine = ["T", "P", "M", "F", "Z", "C", "G"]

def getList(num, part):
	if part == "one":
		if num == "1":
			return p1_one
		if num == "2":
			return p1_two
		if num == "3":
			return p1_three
		if num == "4":
			return p1_four
		if num == "5":
			return p1_five
		if num == "6":
			return p1_six
		if num == "7":
			return p1_seven
		if num == "8":
			return p1_eight
		if num == "9":
			return p1_nine
	else:
		if num == "1":
			return one
		if num == "2":
			return two
		if num == "3":
			return three
		if num == "4":
			return four
		if num == "5":
			return five
		if num == "6":
			return six
		if num == "7":
			return seven
		if num == "8":
			return eight
		if num == "9":
			return nine
		

def partone():
	for line in lines:
		move = line.split(' ')[1]
		fromm = line.split(' ')[3]
		to = line.split(' ')[5].rstrip()

		for i in range(int(move)):
			arr = getList(fromm, "one")
			arr_dest = getList(to, "one")
		
			arr_dest.append(arr[-1])
			del arr[-1]

	print(p1_one[-1], p1_two[-1], p1_three[-1], p1_four[-1], p1_five[-1], p1_six[-1], p1_seven[-1], p1_eight[-1], p1_nine[-1])

def parttwo():
	for line in lines:
		move = line.split(' ')[1]
		fromm = line.split(' ')[3]
		to = line.split(' ')[5].rstrip()

		arr = getList(fromm, "two")
		arr_dest = getList(to, "two")

		for i in range(int(move),0, -1):
			arr_dest.append(arr[-i])
			del arr[-i]

	print(one[-1], two[-1], three[-1], four[-1], five[-1], six[-1], seven[-1], eight[-1], nine[-1])

partone()
parttwo()
