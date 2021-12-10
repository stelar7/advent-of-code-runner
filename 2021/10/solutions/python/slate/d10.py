import sys

input = sys.stdin.readlines()

invalid = []

def match(open, close):
	if open == "(":
		return ")" == close
	if open == "{":
		return "}" == close
	if open == "<":
		return ">" == close
	if open == "[":
		return "]" == close


for line in input:
	open = []
	for brace in line:
		last = open[len(open) -1]

		if brace == "}" or brace == "]" or brace == ">" or brace == ")":
			if match(last, brace):
				open = open[:-1]
			else:
				invalid.append(char)
