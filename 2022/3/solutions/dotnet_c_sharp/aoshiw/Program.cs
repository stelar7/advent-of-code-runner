var span = Console.In.ReadToEnd().AsSpan().Trim();
int sum1 = 0, sum2 = 0;
var enumerator = span.EnumerateLines();
while (enumerator.MoveNext())
{
    var e1 = enumerator.Current;
    Part1(e1, ref sum1);
    enumerator.MoveNext();
    var e2 = enumerator.Current;
    Part1(e2, ref sum1);
    enumerator.MoveNext();
    var e3 = enumerator.Current;
    Part1(e3, ref sum1);
    if (e2.Length > e1.Length)
    {
        var temp = e1;
        e1 = e2;
        e2 = temp;
    }
    if (e3.Length > e1.Length)
    {
        var temp = e1;
        e1 = e3;
        e3 = temp;
    }
    foreach (var c in e1)
    {
        if (e2.Contains(c) && e3.Contains(c))
        {
            sum2 += char.IsLower(c)
                ? c - 'a' + 1
                : c - 'A' + 1 + 26;
            break;
        };
    }
}
Console.WriteLine(sum1);
Console.WriteLine(sum2);

void Part1(ReadOnlySpan<char> item, ref int sum)
{
    var p1 = item.Slice(0, item.Length / 2);
    var p2 = item.Slice(item.Length / 2);
    foreach (var c in p1)
    {
        if (p2.Contains(c))
        {
            sum += char.IsLower(c)
                ? c - 'a' + 1
                : c - 'A' + 1 + 26;
            break;

        };
    }
}