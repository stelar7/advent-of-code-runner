var input = Console.In.ReadToEnd().AsSpan().Trim();

int horizontal = 0, depth = 0, aimdepth = 0;
foreach (var item in input.EnumerateLines())
{
    int num = int.Parse(item.Slice(item.IndexOf(' ') + 1));
    if (item.StartsWith("down", StringComparison.OrdinalIgnoreCase))
    {
        aimdepth += num;
    }
    else if (item.StartsWith("up", StringComparison.OrdinalIgnoreCase))
    {
        aimdepth -= num;
    }
    else
    {
        horizontal += num;
        depth += (aimdepth * num);
    }
}

Console.WriteLine(horizontal * aimdepth);
Console.WriteLine(horizontal * depth);
