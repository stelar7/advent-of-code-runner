var span = Console.In.ReadToEnd().AsSpan().TrimEnd();
int part1 = 0, part2 = 0;
foreach (var item in span.EnumerateLines())
{
    var index = item.IndexOf(',');
    ToNums(item.Slice(0, index), out var from1, out var to1);
    ToNums(item.Slice(index + 1), out var from2, out var to2);
    if ((from1.IsInRange(from2, to2 + 1) && to1.IsInRange(from2, to2 + 1)) || (from2.IsInRange(from1, to1 + 1) && to2.IsInRange(from1, to1 + 1)))
    {
        part1++;
    }
    if (from1.IsInRange(from2, to2 + 1) || to1.IsInRange(from2, to2 + 1) || from2.IsInRange(from1, to1 + 1) || to2.IsInRange(from1, to1 + 1))
    {
        part2++;
    }
}

Console.WriteLine(part1);
Console.WriteLine(part2);

static void ToNums(ReadOnlySpan<char> span, out int from, out int to)
{
    var index = span.IndexOf('-');
    from = int.Parse(span.Slice(0, index));
    to = int.Parse(span.Slice(index + 1));
}

public static class Extensions
{
    public static bool IsInRange<T>(this T item, T minInclusive, T maxExclusive) where T : IComparable<T>
        => item.CompareTo(minInclusive) >= 0 && item.CompareTo(maxExclusive) < 0;
}
