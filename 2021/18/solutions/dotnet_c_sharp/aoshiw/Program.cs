var span = Console.In.ReadToEnd().AsSpan().Trim();

var numbers = new List<SnailNumber>();
foreach (var item in span.EnumerateLines())
{
    numbers.Add(SnailNumber.Parse(item));
}
Console.WriteLine(numbers.Aggregate((x, y) => x + y).GetMagnitude());
int max = 0;
for (int i1 = 0; i1 < numbers.Count; i1++)
{
    var sn1 = numbers[i1];
    for (int i2 = 0; i2 < numbers.Count; i2++)
    {
        if (i1 == i2)
            continue;
        var sn2 = numbers[i2];
        var magnitude = (sn1 + sn2).GetMagnitude();
        if (magnitude > max)
            max = magnitude;
        magnitude = (sn2 + sn1).GetMagnitude();
        if (magnitude > max)
            max = magnitude;
    }
}
Console.WriteLine(max);