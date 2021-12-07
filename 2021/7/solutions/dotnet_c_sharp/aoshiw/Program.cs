var span = Console.In.ReadToEnd().AsSpan().Trim();

var nums = new List<int>();
int min = int.MaxValue, max = int.MinValue;
foreach (var item in span.EnumerateSlices(","))
{
    var num = int.Parse(item);
    if (num < min)
    {
        min = num;
    }
    else if (num > max)
    {
        max = num;
    }
    nums.Add(num);
}
var input = (nums, min, max);
Console.WriteLine(Part(input, x => x));

static int Part((List<int> Nums, int Min, int Max) input, Func<int, int> func)
{
    int fuelMin = int.MaxValue;
    for (int i = input.Min; i < input.Max; i++)
    {
        int fuel = 0;
        foreach (var item in input.Nums)
        {
            fuel += func(Math.Abs(i - item));
            if (fuel > fuelMin)
            {
                break;
            }
        }
        if (fuel < fuelMin)
            fuelMin = fuel;
    }
    return fuelMin;
}

Console.WriteLine(Part(input, x =>
{
    var fuel = 0;
    while (x != 0)
    {
        fuel += x--;
    }
    return fuel;
}));
