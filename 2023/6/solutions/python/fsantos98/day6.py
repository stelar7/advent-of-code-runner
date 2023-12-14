import sys 

def loadData():
    file = sys.stdin.read().strip().split("\n")

    time = [x for x in file[0].split(":")[1].strip().split(" ") if x != ""]
    distance = [x for x in file[1].split(":")[1].strip().split(" ") if x != ""]
    races = [(int(time[x]), int(distance[x])) for x in range(len(time))]

    return races

def runRace(time, record_distance):
    record_races = 0
    for hold_time in range(time + 1):
        record_races += (time - hold_time) * hold_time > record_distance
    return record_races

def runRaceSpeed(time, record_distance):
    time -= 1
    start = 0
    end = time

    hold_timer = end // 2 + 1

    while True:
        distance = (time - hold_timer) * hold_timer
        if distance >= record_distance: end = hold_timer
        elif distance < record_distance: start = hold_timer

        hold_timer = start + ((end - start) // 2)

        if end - start <= 1: break
    return time - start * 2

def part1(data):
    total = 1
    for i in range(len(data)):
        total *= runRace(data[i][0], data[i][1])
    return total

def part2(data):
    time = int("".join([str(x[0]) for x in data]))
    distance = int("".join([str(x[1]) for x in data]))
    return runRaceSpeed(time, distance)

if __name__ == "__main__":
    data = loadData()
    print(part1(data))
    print(part2(data))
