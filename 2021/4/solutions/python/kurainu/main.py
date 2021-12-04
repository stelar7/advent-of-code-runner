import re, sys

Lines = ''.join(sys.stdin.readlines()).split("\n\n")

num_Boards = len(Lines)
RNG_Numbers = Lines[0].split(",")


class Board:
    def __init__(self, boarddata):
        self.data = boarddata

    def Mark_Number(self, num):
        for i in range(5):
            for j in range(5):
                if self.data[i][j] == num:
                    self.data[i][j] = "marked"

    def Bingo(self):
        for i in range(5):
            if self.data[i][0] == "marked" and self.data[i][1] == "marked" and self.data[i][2] == "marked" and self.data[i][3] == "marked" and self.data[i][4] == "marked":
                return True
            if self.data[0][i] == "marked" and self.data[1][i] == "marked" and self.data[2][i] == "marked" and self.data[3][i] == "marked" and self.data[4][i] == "marked":
                return True

        return False

    def Get_Sum(self):
        sum = 0
        for i in range(5):
            for j in range(5):
                if self.data[i][j] != "marked":
                    sum += int(self.data[i][j])
        return sum

Boards = []

def Solve():
  First_Total = 0
  Last_total = 0
  last_num = 0
  first_Bingo_found = False
  # Populate Boards Array
  for i in range(1, num_Boards):
      temp = []
      for row in Lines[i].split("\n"):
        r = re.compile("(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)")
        match = r.findall(row)
        if len(match) == 0:
          continue

        temp.append(list(match[0]))
      Boards.append(Board(temp))

  # Draw the RNG Numbers
  for num in RNG_Numbers:
      for board in Boards:
        if not first_Bingo_found:
          board.Mark_Number(num)

        if board.Bingo() == True:
          if first_Bingo_found == False:
            First_Total = board.Get_Sum() * int(num)
            first_Bingo_found = True
          continue

        if first_Bingo_found:
          board.Mark_Number(num)

        Last_total = board
        last_num = num
          
  return First_Total, Last_total.Get_Sum() * int(last_num)

Solves = Solve()

print(Solves[0])
print(Solves[1])
