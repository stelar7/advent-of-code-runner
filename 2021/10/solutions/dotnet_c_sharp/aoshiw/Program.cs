var span = Console.In.ReadToEnd().AsSpan().Trim();

var part1 = 0L;
var fr = new Stack<char>();
var l = new List<long>();
foreach (var line in span.EnumerateLines())
{
    fr.Clear();
    var ib = false;
    foreach (var item in line)
    {
        if (item is '(' or '<' or '[' or '{')
        {
            fr.Push(item switch
            {
                '(' => ')',
                '<' => '>',
                '[' => ']',
                '{' => '}',
                _ => throw new ArgumentException(null, nameof(span))
            });
        }
        else
        {
            if (!fr.TryPop(out var iteem) || iteem != item)
            {
                ib = true;
                part1 += item switch
                {

                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => throw new ArgumentException(null, nameof(span))
                };
            }
        }
    }
    if (!ib)
    {
        var temp = 0L;
        while (fr.TryPop(out var item))
        {
            temp = temp * 5 + item switch
            {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => throw new ArgumentException(null, nameof(span))
            };
        }
        l.Add(temp);
    }
}
l.Sort();
Console.WriteLine(part1);
Console.WriteLine(l[l.Count / 2]);
