using renkon;

int DoPartA()
{
    var heightMap = Utils.InputToStringArray("9")
        .Where(s => !string.IsNullOrEmpty(s))
        .Select(s => s.ToCharArray().Select(c => c - '0').ToArray())
        .ToArray();
    var riskLevelSum = 0;
    for (int i = 0; i < heightMap[0].Length; i++)
    {
        for (int j = 0; j < heightMap.Length; j++)
        {
            if (IsLowerPoint(heightMap, j, i))
            {
                riskLevelSum += heightMap[j][i] + 1;
            }
        }
    }

    return riskLevelSum;
}

int DoPartB()
{
    var heightMap = Utils.InputToStringArray("9")
        .Where(s => !string.IsNullOrEmpty(s))
        .Select(s => s.ToCharArray().Select(c => c - '0').ToArray())
        .ToArray();
    var visitMap = heightMap.Select(sm => new bool[sm.Length]).ToArray();
    SetHeightAsVisited(9, heightMap, visitMap);

    var basinLengths = new List<int>();
    for (int i = 0; i < heightMap[0].Length; i++)
    {
        for (int j = 0; j < heightMap.Length; j++)
        {
            if (!visitMap[j][i])
            {
                basinLengths.Add(GetBasinLengthFromPoint(heightMap, visitMap, i, j));
            }
        }
    }

    return basinLengths.OrderByDescending(i => i).Take(3).Aggregate(1, (a, b) => a * b);
}

void SetHeightAsVisited(int height, int[][] heightMap, bool[][] visitMap)
{
    for (int i = 0; i < heightMap[0].Length; i++)
    {
        for (int j = 0; j < heightMap.Length; j++)
        {
            if (heightMap[j][i] == height)
            {
                visitMap[j][i] = true;
            }
        }
    }
}

int GetBasinLengthFromPoint(int[][] map, bool[][] visits, int y, int x)
{
    var lengthCount = 1;
    visits[x][y] = true;

    if (x > 0 && !visits[x - 1][y])
    {
        lengthCount += GetBasinLengthFromPoint(map, visits, y, x - 1);
    }

    if (x < map.Length - 1 && !visits[x + 1][y])
    {
        lengthCount += GetBasinLengthFromPoint(map, visits, y, x + 1);
    }

    if (y > 0 && !visits[x][y - 1])
    {
        lengthCount += GetBasinLengthFromPoint(map, visits, y - 1, x);
    }

    if (y < map[0].Length - 1 && !visits[x][y + 1])
    {
        lengthCount += GetBasinLengthFromPoint(map, visits, y + 1, x);
    }

    return lengthCount;
}

bool IsLowerPoint(int[][] map, int x, int y)
    => !((x > 0 && map[x - 1][y] <= map[x][y]) ||
        (x < map.Length - 1 && map[x + 1][y] <= map[x][y]) ||
        (y > 0 && map[x][y - 1] <= map[x][y]) ||
        (y < map[0].Length - 1 && map[x][y + 1] <= map[x][y]));

Console.WriteLine(DoPartA() + "\n" + DoPartB());