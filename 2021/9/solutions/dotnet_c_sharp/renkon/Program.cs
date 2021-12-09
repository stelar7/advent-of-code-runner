using renkon;

int DoPartA()
{
    var heightMap = Utils.InputToStringArray("9").Select(s => s.ToCharArray().Select(c => c - '0').ToArray()).ToArray();
    var riskLevelSum = 0;
    for (int i = 0; i < heightMap[0].Length; i++)
    {
        for (int j = 0; j < heightMap.Length; j++)
        {
            if (IsLowerPoint(heightMap, i, j))
            {
                riskLevelSum += heightMap[i][j] + 1;
            }
        }
    }

    return riskLevelSum;
}

int DoPartB()
{
    var heightMap = Utils.InputToStringArray("9").Select(s => s.ToCharArray().Select(c => c - '0').ToArray()).ToArray();
    var visitMap = heightMap.Select(sm => new bool[sm.Length]).ToArray();
    SetHeightAsVisited(9, heightMap, visitMap);

    var basinLengths = new List<int>();
    for (int i = 0; i < heightMap[0].Length; i++)
    {
        for (int j = 0; j < heightMap.Length; j++)
        {
            if (!visitMap[i][j])
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
            if (heightMap[i][j] == height)
            {
                visitMap[i][j] = true;
            }
        }
    }
}

int GetBasinLengthFromPoint(int[][] map, bool[][] visits, int x, int y)
{
    var lengthCount = 1;
    visits[x][y] = true;

    if (x > 0 && !visits[x - 1][y])
    {
        lengthCount += GetBasinLengthFromPoint(map, visits, x - 1, y);
    }

    if (x < map[0].Length - 1 && !visits[x + 1][y])
    {
        lengthCount += GetBasinLengthFromPoint(map, visits, x + 1, y);
    }

    if (y > 0 && !visits[x][y - 1])
    {
        lengthCount += GetBasinLengthFromPoint(map, visits, x, y - 1);
    }

    if (y < map.Length - 1 && !visits[x][y + 1])
    {
        lengthCount += GetBasinLengthFromPoint(map, visits, x, y + 1);
    }

    return lengthCount;
}

bool IsLowerPoint(int[][] map, int x, int y)
    => !((x > 0 && map[x - 1][y] <= map[x][y]) ||
        (x < map[0].Length - 1 && map[x + 1][y] <= map[x][y]) ||
        (y > 0 && map[x][y - 1] <= map[x][y]) ||
        (y < map.Length - 1 && map[x][y + 1] <= map[x][y]));

Console.WriteLine(DoPartA() + "\n" + DoPartB());