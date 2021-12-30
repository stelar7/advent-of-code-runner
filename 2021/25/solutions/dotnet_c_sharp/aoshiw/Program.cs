var map = new List<char[]>(138);
var span = Console.In.ReadToEnd().AsSpan().Trim();
foreach (var item in span.EnumerateLines())
{
    map.Add(item.ToArray());
}
int i = 0, row = map.Count, column = map[0].Length;
for (var isMove = true; isMove; i++)
{
    isMove = false;
    for (int r = 0; r < row; r++)
    {
        var tempRow = map[r];
        var first = tempRow[0];
        for (int c = 0; c < map[0].Length; c++)
        {
            var isLast = c == column - 1;
            if (tempRow[c] == '>' && (isLast ? first : tempRow[c + 1]) == '.')
            {
                tempRow[c] = '.';
                isMove = true;
                tempRow[isLast ? 0 : ++c] = '>';

            }
        }
    }
    for (int c = 0; c < column; c++)
    {
        var first = map[0][c];
        for (int r = 0; r < row; r++)
        {
            var isLast = r == row - 1;
            if (map[r][c] == 'v' && (isLast ? first : map[r + 1][c]) == '.')
            {
                map[r][c] = '.';
                isMove = true;
                map[isLast ? 0 : ++r][c] = 'v';
            }
        }
    }
}
Console.WriteLine(i);