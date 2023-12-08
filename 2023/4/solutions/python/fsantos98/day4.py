import sys

file = sys.stdin.read().strip().split("\n")

def part1():
    total = 0
    for item in file:
        parse = item[item.index(':')+2:].split("|")
        my_numbers = parse[0].strip().split(" ")
        winning_numbers = parse[1].strip().split(" ")

        points = 0
        for x in my_numbers:
            if x != "" and x in winning_numbers:
                points = points * 2 or 1
        total += points
    return total

def part2():
    scratches_base = []
    scratches_list = []

   
    for item in file:
        parse = item[item.index(':')+2:].split("|")
        
        my_numbers = parse[0].strip().split(" ")
        winning_numbers = parse[1].strip().split(" ")
        
        scratch_card = [my_numbers, winning_numbers]
        
        scratches_base.append(scratch_card)
        scratches_list.append(1)
    
    return processScratches(scratches_base, scratches_list)

def sumArrays(scratches_indexes, solution, factor = 1):
    for i in range(len(scratches_indexes)):
        scratches_indexes[i] += solution[i] * factor
    return scratches_indexes

def solve(SOLVES, scratches_indexes, my_numbers, winning_numbers, scratch_index):
    
    if len(SOLVES[scratch_index][0]) > 0:
        factor = scratches_indexes[scratch_index]
        scratches_indexes[scratch_index] -= factor
        sumArrays(scratches_indexes, SOLVES[scratch_index][0], factor)
        return SOLVES[scratch_index][1] * factor

    points = 0
    scratches_indexes[scratch_index + points] -= 1
    scratch_solution = [0 for _ in range(len(scratches_indexes))]
    for x in my_numbers:
        if x != "" and x in winning_numbers:
            points += 1
            scratches_indexes[scratch_index + points] += 1
            scratch_solution[scratch_index + points] += 1


    SOLVES[scratch_index][0] = scratch_solution
    SOLVES[scratch_index][1] = points

    return points

def processScratches(scratches, scratches_indexes):
    SOLVES = [[[],[]] for _ in range(len(scratches_indexes))]

    total = len(scratches_indexes)
    n = 0
    while(sum(scratches_indexes) > 0):

        my_numbers = scratches[n][0]
        winning_numbers = scratches[n][1]
        
        total += solve(SOLVES, scratches_indexes, my_numbers, winning_numbers, n)

        if scratches_indexes[n] == 0:
            n += 1
       
    return total

print(part1())
print(part2())