import sys

SEEDS = []
SEEDS_PART_2 = []
DATA = {
    "seed-to-soil": [],
    "soil-to-fertilizer": [],
    "fertilizer-to-water": [],
    "water-to-light": [],
    "light-to-temperature": [],
    "temperature-to-humidity": [],
    "humidity-to-location": []
}
CACHE = {}

def run_seed_group(seed_group, level = "seed"):
    if level+str(seed_group) in CACHE:
        return CACHE[level+str(seed_group)]
    
    seed_location_start = int(seed_group[0])
    seed_location_end = int(seed_group[0]) + int(seed_group[1]) - 1
    level += "-"
    
    min_seed = 9999999999
    min_seed_final = 9999999999
    DATA_TO_USE = {}
    use = False
    if level == "-":
        DATA_TO_USE = DATA
    else:
        for key in DATA:
            if level in key:
                use = True
            if use:
                DATA_TO_USE[key] = DATA[key]

    for key in DATA_TO_USE:
        layer = key.split("-")[0]
        for map_set in DATA_TO_USE[key]:
            range_destination = int(map_set[0])
            range_source = int(map_set[1])
            range_length = int(map_set[2]) - 1
            range_end = range_source + range_length
            
            if(seed_location_start > range_end or seed_location_end < range_source):
                continue

            if(seed_location_start >= range_source and seed_location_end <= range_end):
                seed_location_start = range_destination + (seed_location_start - range_source)
                seed_location_end = range_destination + (seed_location_end - range_source)
                
                if key == "humidity-to-location":
                    return min(seed_location_start, min_seed_final)
                
                min_seed = run_seed_group([seed_location_start, seed_location_end - seed_location_start + 1], key.split("-")[2].split(" ")[0])
                CACHE[layer + str([seed_location_start, seed_location_end - seed_location_start + 1])] = min_seed
                
                return min(min_seed_final, min_seed)
                
            elif(seed_location_start < range_source and seed_location_end <= range_end):
                run1 = run_seed_group([seed_location_start, range_source - seed_location_start], layer)
                CACHE[layer + str([seed_location_start, range_source - seed_location_start])] = run1

                run2 = run_seed_group([range_source, seed_location_end - range_source + 1], layer)
                CACHE[layer + str([range_source, seed_location_end - range_source + 1])] = run2

                return min(min_seed_final, run1, run2)
            
            elif(seed_location_start >= range_source and seed_location_end > range_end):
                run1 = run_seed_group([range_end + 1, seed_location_end - range_end], layer)
                CACHE[layer + str([range_end + 1, seed_location_end - range_end])] = run1

                run2 = run_seed_group([seed_location_start, range_end - seed_location_start + 1], layer)
                CACHE[layer + str([seed_location_start, range_end - seed_location_start + 1])] = run2
                
                return min(min_seed_final, run1, run2)
                
    return min(seed_location_start, min_seed_final)

def creteSeedGroups():
    global SEEDS
    seeds_group_list = [[SEEDS[0][x], SEEDS[0][x + 1]] for x in range(0, len(SEEDS[0]), 2)]
    return seeds_group_list

def loadData():
    global SEEDS
    global SEEDS_PART_2
    stage = ""
    file = sys.stdin.read().strip().split("\n")

    for item in file:
        items = " ".join(item.strip().split(" "))
        if not items:
            continue

        mapper = items.replace("map", "").replace(":","").strip().split(" ")
        if mapper[0] in DATA.keys() or mapper[0] == "seeds":
            stage = mapper[0]
            if stage == "seeds":
                SEEDS = [mapper[1:]]
            continue

        range_destination = mapper[0]
        range_source = mapper[1]
        range_length = mapper[2]
        DATA[stage].append([range_destination, range_source, range_length])
    
    SEEDS_PART_2 = creteSeedGroups()

def part1():
    global SEEDS
    seed_location = -1
    min_seed = -1
    
    for seed_set in SEEDS:
        for seed in seed_set:
            seed_location = int(seed)
            for key in DATA:
                for map_set in DATA[key]:
                    range_destination = int(map_set[0])
                    range_source = int(map_set[1])
                    range_length = int(map_set[2])
                    
                    if(seed_location >= range_source and seed_location <= range_source + range_length):
                        seed_location = range_destination + (seed_location - range_source)
                        break

            if seed_location < min_seed or min_seed == -1:
                min_seed = seed_location

    return min_seed

def part2():
    results = []
    for group in SEEDS_PART_2:
        results.append(run_seed_group(group))
    return min(results)


if __name__ == "__main__":
    loadData()

    print(part1())
    print(part2())

   