var span = Console.In.ReadToEnd().AsSpan().Trim();

var m = new (int, int)[1000, 1000];
Span<int> arr = stackalloc int[4];
foreach (var item in span.EnumerateLines())
{
    var enumerator = item.EnumerateSlices(",-> ");
    for (int i = 0; enumerator.MoveNext(); i++)
    {
        arr[i] = int.Parse(enumerator.Current);
    }
    if (arr[0] == arr[2] || arr[1] == arr[3])
    {
        int r1 = Math.Min(arr[0], arr[2]);
        var r2 = Math.Max(arr[0], arr[2]);
        int c0 = Math.Min(arr[1], arr[3]);
        int c2 = Math.Max(arr[1], arr[3]);
        for (; r1 <= r2; r1++)
        {
            for (int c1 = c0; c1 <= c2; c1++)
            {
                m[r1, c1].Item1++;
            }
        }
    }
    else
    {
        int r = arr[0];
        int c = arr[1];
        while (r != arr[2] && c != arr[3])
        {
            m[r, c].Item2++;
            if (arr[0] < arr[2])
            {
                r++;
            }
            else
            {
                r--;
            }
            if (arr[1] < arr[3])
            {
                c++;
            }
            else
            {
                c--;
            }
        }
        m[r, c].Item2++;
    }
}
int part1 = 0, part2 = 0;
foreach (var item in m)
{
    if (item.Item1 > 1)
    {
        part1++;
        part2++;
    }
    else if (item.Item2 > 1)
        part2++;
}
return (part1, part2);

Console.WriteLine(part1);
Console.WriteLine(part2);
