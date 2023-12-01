import sys

input_lines = sys.stdin.readlines()


sum_part_1 = 0

for line in input_lines:
    digits = [char for char in line if char.isdigit()]
    
    if digits:
        first_digit = digits[0]
        last_digit = digits[-1]

        calibration_value = int(first_digit + last_digit)
        
        sum_part_1 += calibration_value

print(sum_part_1)


# --- Part Two ---

sum_part_2 = 0
digit_names = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

def get_digit(char):
    if char.isdigit():
        return int(char)
    
    for j, name in enumerate(digit_names):
        if line[i:(i+len(name))] == name:
            return j + 1
    
    return None

for line in input_lines:
    first_digit = None
    last_digit = None

    for i in range(len(line)):
        current_digit = get_digit(line[i])
        if current_digit is not None:
            if first_digit is None:
                first_digit = current_digit
            last_digit = current_digit

    sum_part_2 += first_digit * 10 + last_digit

print(sum_part_2)
