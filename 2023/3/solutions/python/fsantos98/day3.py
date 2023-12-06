POSSIBLE_POS = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [0, -1],
    [1, 1],
    [1, 0],
    [1, -1],
]

def loadData():
    data = []
    with open("day3.txt") as file:
        n = 0
        for item in file:
            if len(data) <= n:
                data.append([])
            for char in item:
                if(char != "\n"):
                    data[n].append(char)
            n += 1

    return data

def getSymbolsPos(data):
    symbols_pos = []
    for i in range(len(data)):
        for j in range(len(data[i])):
            if(data[i][j] != "." and not data[i][j].isdigit()):
                symbols_pos.append([i, j])
    return symbols_pos

def searchForSymbol(data, x, y):
    value = data[x][y]
    if(value.isdigit()):

        # right
        n = 1
        while(True):
            right_value = y + n
            if(right_value > len(data[x]) - 1):
                break

            if(data[x][right_value].isdigit()):
                value += data[x][right_value]
                data[x][right_value] = "."
            else: 
                break
            n += 1
        # left
        n = -1
        while(True):
            left_value = y + n
            if(left_value > len(data[x]) - 1):
                break
            
            if data[x][left_value].isdigit():
                value = data[x][left_value] + value
                data[x][left_value] = "."
            else: 
                break
            n -= 1
    return value

def getNumbersCloseToSymbol(data, symbol):
    total = 0
    total_part2 = 0

    # print(symbol)
    symbol_x = symbol[0]
    symbol_y = symbol[1]

    symbols_found = []
    for diff_x, diff_y in POSSIBLE_POS:
        if symbol_x + diff_x < 0 or symbol_x + diff_x > len(data) - 1:
            continue
        if symbol_y + diff_y < 0 or symbol_y + diff_y > len(data) - 1:
            continue
        
        # print(diff_x, diff_y)
        x = symbol_x + diff_x
        y = symbol_y + diff_y
        f_value = searchForSymbol(data, x, y)
        if(f_value and f_value.isdigit()):
            total += int(f_value)
            if data[symbol_x][symbol_y] == "*":
                symbols_found.append(f_value)
    
    if(len(symbols_found) == 2):
        total_part2 = int(symbols_found[0]) * int(symbols_found[1])

    return total, total_part2

def getTotalNumbers(data, symbols_pos):
    total = 0
    total_part2 = 0
    for symbol in symbols_pos:
        symbol_total, symbol_racio = getNumbersCloseToSymbol(data, symbol)
        total += symbol_total
        total_part2 += symbol_racio
    
    return total, total_part2

if __name__ == "__main__":
    data = loadData()
    symbols_pos = getSymbolsPos(data)
    total, total_part2 = getTotalNumbers(data, symbols_pos)

    print(total)
    print(total_part2)
