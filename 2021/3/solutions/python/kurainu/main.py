import sys
Lines = sys.stdin.readlines()

gamma_rate = ""
epsilon_rate = ""

for i in range(0,len(Lines[0])-1):
    count_zero = 0
    count_one = 0

    for line in Lines:
        digit = line[i:i+1]
        if digit == '1':
            count_one  += 1
        if  digit == '0':
            count_zero += 1
            
    if count_one > count_zero:
        gamma_rate += '1'
    else:
        gamma_rate += '0'

epsilon_rate = ''.join('1' if x == '0' else '0' for x in gamma_rate)

print(int(gamma_rate,2) * int(epsilon_rate,2))

#Day 3 Part 2

def get_rating(pos, numbers, mode):
    count_zero = 0
    count_one = 0
    one_numbers = []
    zero_numbers = []

    if len(numbers) == 1:
        return numbers[0]

    for num in numbers:
        digit = num[pos:pos+1]
        if digit == '1':
            count_one += 1
            one_numbers.append(num)
        if digit == '0':
            count_zero += 1
            zero_numbers.append(num)
    if count_one > count_zero:
        return get_rating(pos+1,one_numbers,"oxygen") if mode == "oxygen" else get_rating(pos+1,zero_numbers,"CO2") 
    elif count_zero > count_one:
        return get_rating(pos+1,zero_numbers,"oxygen") if mode == "oxygen" else get_rating(pos+1,one_numbers,"CO2")
    elif count_one == count_zero:
        return get_rating(pos+1,one_numbers,"oxygen") if mode == "oxygen" else get_rating(pos+1,zero_numbers,"CO2")

print(int(get_rating(0, Lines, "oxygen"),2) * int(get_rating(0, Lines, "CO2"),2))

