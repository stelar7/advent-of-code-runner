import sys
import string

input = sys.stdin.readlines()

alphabet = ""

alphabet += string.ascii_lowercase
alphabet += string.ascii_uppercase

priority = list(alphabet)

total_pri = 0
total_pri_two = 0

groups = 0
group_chars = []

for line in input:
	groups += 1
	if groups > 3:
		group_chars.clear()
		groups = 1
	group_chars.append(line)

	if groups == 3:
		already = []
		for new_char in group_chars[0]:
			if new_char in group_chars[1] and new_char in group_chars[2] and new_char not in already:
				already.append(new_char)
				for j in range(len(priority)):
					if new_char == priority[j]:
						total_pri_two += (j+1)
						break
						
				


	half = len(line)//2
	first_half = line[:half]
	second_half = line[half:].rstrip()
	already = []
	
	for char in first_half:
		if char in second_half and char not in already:
			already.append(char)
			for i in range(len(priority)):
				if char == priority[i]:
					total_pri += (i + 1)
					break


print(total_pri)
print(total_pri_two)
#print(priority)			
