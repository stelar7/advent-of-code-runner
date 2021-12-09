using System.Drawing;

var span = Console.In.ReadToEnd().AsSpan().Trim();

int part1 = 0;

var PointOffset = new Size[] { new(1, 0), new(0, 1), new(-1, 0), new(0, -1) };
var map = new Map2D<char>();
foreach (var item in span.EnumerateLines())
{
    map.AddRow(item);
}
for (Point point = new(); point.X < map.Row; point.X++)
{
    for (point.Y = 0; point.Y < map.Column; point.Y++)
    {
        var isNotBreak = true;
        foreach (var item in PointOffset)
        {
            var newPoint = point + item;
            if (newPoint.X.IsInRange(0, map.Row) && newPoint.Y.IsInRange(0, map.Column) && map[point] >= map[newPoint])
            {
                isNotBreak = false;
                break;
            }
        }
        if (isNotBreak)
        {
            part1 += (int)char.GetNumericValue(map[point]) + 1;
        }
    }
}
Console.WriteLine(part1);
Span<int> max = stackalloc int[3];
for (int r = 0; r < map.Row; r++)
{
    for (int c = 0; c < map.Column; c++)
    {
        var val = Rec(r, c, map);
        if (val > max[0])
        {
            max[0] = val;
            max.Sort();
        }
    }
}
Console.WriteLine(max[0] * max[1] * max[2]);

int Rec(int r, int c, Map2D<char> map)
{
    var sum = 0;
    var tack = new Stack<Point>();
    tack.Push(new(r, c));
    while (tack.TryPop(out var point))
    {
        foreach (var item in PointOffset)
        {
            var newPoint = point + item;
            if (newPoint.X.IsInRange(0, map.Row) && newPoint.Y.IsInRange(0, map.Column) && map[newPoint] != '9')
            {
                sum++;
                tack.Push(newPoint);
                map[newPoint] = '9';
            }
        }
    }
    return sum;
}
