var input = Console.In.ReadToEnd().AsSpan().Trim();
Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));

int Part1(ReadOnlySpan<char> span)
{
    var num = new List<int>();
    foreach (var item in span.EnumerateLines())
    {
        num.Add(int.Parse(item));
    }
    int count = 0;
    var max = num[0];
    for (int i = 1; i < num.Count; i++)
    {
        if (num[i] > num[i - 1])
            count++;
    }
    return count;
}

int Part2(ReadOnlySpan<char> span)
{
    var num = new List<int>();
    foreach (var item in span.EnumerateLines())
    {
        num.Add(int.Parse(item));
    }
    int count = 0;
    var sum = num[0] + num[1] + num[2];
    for (int i = 1; i < num.Count - 2; i++)
    {
        var item = num[i] + num[i + 1] + num[i + 2];
        if (item > sum)
            count++;
        sum = item;
    }
    return count;
}
