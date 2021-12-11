import sys

input = sys.stdin.readlines()

def matchBrace(open, close):
    if open == "(":
        return close == ")"
    if open == "[":
        return close == "]"
    if open == "{":
        return close == "}"
    if open == "<":
        return close == ">"

def matching(open):
    if open == "(":
        return ")"
    if open == "[":
        return "]"
    if open == "{":
        return "}"
    if open == "<":
        return ">"

def score(current, char):
    if char == ")":
        points = 1
    if char == "]":
        points = 2
    if char == "}":
        points = 3
    if char == ">":
        points = 4
    return (current * 5) + points

illegal = []
incomplete = []

def partOne():
    for line in input:
        open = []

        for brace in line.strip('\n'):
            if len(open) > 0:
                latest = open[-1]
            if brace == "(" or brace == "[" or brace == "{" or brace == "<":
                open.append(brace)
            else:
                if matchBrace(latest, brace):
                    open = open[:-1]
                    incomplete.append(line)
                else:
                    illegal.append(brace)
                    while line in incomplete:
                        incomplete.remove(line)
                    break
        

partOne()

scores = []

incomplete = list(dict.fromkeys(sorted(incomplete)))


for line in incomplete:
    current = 0
    open = []
    openings = ['(', '[', '{', '<']
    needed = ""

    for brace in line.strip('\n'):
        if len(open) > 0:
            latest = open[-1]
        if brace == "(" or brace == "[" or brace == "{" or brace == "<":
            open.append(brace)
        else:
            if matchBrace(latest, brace):
                open = open[:-1]
    
    for brace in open[::-1]:
        match = matching(brace)
        needed += match

        current = score(current, match)
    scores.append(current)

scores = list(dict.fromkeys(sorted(scores)))

score = 0

for char in illegal:
    if char == ")":
        score += 3
    elif char == "]":
        score += 57
    elif char == "}":
        score += 1197
    elif char == ">":
        score += 25137

print(score)
print(scores[int(len(scores)/2)])