using System.Drawing;

var span = Console.In.ReadToEnd().AsSpan().Trim();

int part1 = 0;

var PointOffset = new Size[] { new(1, 0), new(0, 1), new(-1, 0), new(0, -1) };
var map = new Map2D<char>();
foreach (var item in span.EnumerateLines())
{
    map.AddRow(item);
}
foreach (var point in map.GetPointEnumerator())
{
    var isNotBreak = true;
    foreach (var item in PointOffset)
    {
        var newPoint = point + item;
        if (map.IsInRange(newPoint) && map[point] >= map[newPoint])
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

Console.WriteLine(part1);
Span<int> max = stackalloc int[3];
foreach (var point in map.GetPointEnumerator())
{
    if (map[point] != '9')
    {
        var val = Rec(point, map);
        if (val > max[0])
        {
            max[0] = val;
            max.Sort();
        }
    }
}
Console.WriteLine(max[0] * max[1] * max[2]);

int Rec(Point point, Map2D<char> map)
{
    var sum = 0;
    var tack = new Stack<Point>();
    tack.Push(point);
    while (tack.TryPop(out point))
    {
        foreach (var item in PointOffset)
        {
            var newPoint = point + item;
            if (map.IsInRange(newPoint) && map[newPoint] != '9')
            {
                sum++;
                tack.Push(newPoint);
                map[newPoint] = '9';
            }
        }
    }
    return sum;
}
