using System.Buffers;
var span = Console.In.ReadToEnd().AsSpan().TrimEnd();
int sum1 = 0, sum2 = 0;

SearchValues<char> NiceNumbers = SearchValues.Create("123456789");
string[] NotNiceNumbers = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

foreach (var line in span.EnumerateLines())
{
    int min = line.IndexOfAny(NiceNumbers);
    int max = line.LastIndexOfAny(NiceNumbers);
    if (min is not -1)
        sum1 += ToNum(min, line) * 10 + ToNum(max, line);
    foreach (var line2 in NotNiceNumbers)
    {
        var i = line.IndexOf(line2);
        if (i is -1)
            continue;
        max = int.Max(max, line.LastIndexOf(line2));
        min = min == -1 ? i : int.Min(min, i);
    }
    sum2 += ToNum(min, line) * 10 + ToNum(max, line);
}

Console.WriteLine(sum1);
Console.WriteLine(sum2);

static int ToNum(int index, ReadOnlySpan<char> span)
{
    var c = span[index];
    if (c >= '0' && c <= '9')
        return c - '0';
    return c switch
    {
        'o' => 1,
        't' => span[index + 1] == 'w' ? 2 : 3,
        'f' => span[index + 1] == 'o' ? 4 : 5,
        's' => span[index + 1] == 'i' ? 6 : 7,
        'e' => 8,
        'n' => 9,
    };
}
