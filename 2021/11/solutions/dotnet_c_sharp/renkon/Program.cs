using renkon;

long DoPartA()
{
    var entries = Utils.InputToStringArray("11")
        .Where(s => !string.IsNullOrEmpty(s))
        .Select(s => s.ToCharArray().Select(s => s - '0').ToArray())
        .ToArray();
    var flashCount = 0L;

    for (int i = 0; i < 100; i++)
    {
        flashCount += SimulateStep(entries);
    }

    return flashCount;
}

long DoPartB()
{
    var entries = Utils.InputToStringArray("11")
        .Where(s => !string.IsNullOrEmpty(s))
        .Select(s => s.ToCharArray().Select(s => s - '0').ToArray())
        .ToArray();
    var stepCounter = 0;

    while (!entries.All(e => e.All(i => i == 0)))
    {
        SimulateStep(entries);
        stepCounter++;
    }

    return stepCounter;
}

long SimulateStep(int[][] map)
{
    var flashedMap = map.Select(m => new bool[m.Length]).ToArray();
    var flashCount = PerformFlashes(map, flashedMap);
    ResetFlashed(map, flashedMap);

    return flashCount;
}

long PerformFlashes(int[][] map, bool[][] flashedMap)
{
    var flashes = 0L;
    for (int i = 0; i < map[0].Length; i++)
    {
        for (int j = 0; j < map.Length; j++)
        {
            flashes += PerformFlashInternal(map, flashedMap, i, j);
        }
    }

    return flashes;
}

long PerformFlashInternal(int[][] map, bool[][] flashedMap, int i, int j)
{
    if (i < 0 || i >= map[0].Length || j < 0 || j >= map.Length)
    {
        return 0;
    }

    map[i][j]++;

    if (map[i][j] <= 9 || flashedMap[i][j])
    {
        return 0;
    }

    flashedMap[i][j] = true;
    var flashes = 1L;

    flashes += PerformFlashInternal(map, flashedMap, i - 1, j - 1);
    flashes += PerformFlashInternal(map, flashedMap, i - 1, j);
    flashes += PerformFlashInternal(map, flashedMap, i - 1, j + 1);
    flashes += PerformFlashInternal(map, flashedMap, i, j - 1);
    flashes += PerformFlashInternal(map, flashedMap, i, j + 1);
    flashes += PerformFlashInternal(map, flashedMap, i + 1, j - 1);
    flashes += PerformFlashInternal(map, flashedMap, i + 1, j);
    flashes += PerformFlashInternal(map, flashedMap, i + 1, j + 1);

    return flashes;
}

void ResetFlashed(int[][] map, bool[][] flashedMap)
{
    for (int i = 0; i < map[0].Length; i++)
    {
        for (int j = 0; j < map.Length; j++)
        {
            if (flashedMap[i][j])
            {
                map[i][j] = 0;
            }
        }
    }
}

Console.WriteLine(DoPartA() + "\n" + DoPartB());