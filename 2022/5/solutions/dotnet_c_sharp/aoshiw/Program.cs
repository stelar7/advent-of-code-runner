var span = Console.In.ReadToEnd().AsSpan().TrimEnd();
var part1 = new List<List<char>>();
var enumerator = span.EnumerateLines();
while (enumerator.MoveNext() && !enumerator.Current.IsEmpty)
{
    var item = enumerator.Current;
    var index = item.IndexOf('[');
    if (index == -1)
        continue;
    for (int i = index + 1; i < item.Length; i += 4)
    {
        if (item[i] != ' ')
        {
            index = (i - 1) / 4;
            while (part1.Count <= index)
                part1.Add(new());
            part1[index].Add(item[i]);
        }
    }
}
foreach (var item in part1)
{
    item.Reverse();
}
var part2 = new List<List<char>>(part1.Count);
for (int i = 0; i < part1.Count; i++)
{
    part2.Add(part1[i].ToList());
}
foreach (var item in enumerator)
{
    var s = item.Slice(5);
    var index = s.IndexOf(' ');
    var count = int.Parse(s.Slice(0, index));
    s = s.Slice(index + 6);
    index = s.IndexOf(' ');
    var from = int.Parse(s.Slice(0, index)) - 1;
    index = s.LastIndexOf(' ');
    var to = int.Parse(s.Slice(index + 1)) - 1;

    CrateMover9000(part1[from], part1[to], count);
    CrateMover9001(part2[from], part2[to], count);
}
Console.WriteLine(string.Create(part1.Count, part1, (s, a) =>
{
    for (int i = 0; i < s.Length; i++)
    {
        s[i] = a[i][^1];
    }
}));

 Console.WriteLine(string.Create(part2.Count, part2, (s, a) =>
{
    for (int i = 0; i < s.Length; i++)
    {
        s[i] = a[i][^1];
    }
}));
void BuildString()
void CrateMover9000(List<char> from, List<char> to, int count)
{
    while (count-- != 0)
    {
        to.Add(from[^1]);
        from.RemoveAt(from.Count - 1);
    }
}

void CrateMover9001(List<char> from, List<char> to, int count)
{
    var fromIndex = from.Count - count;
    for (int i = 0; i < count; i++)
    {
        to.Add(from[fromIndex + i]);
    }
    from.RemoveRange(fromIndex, count);
}
