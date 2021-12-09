var span = Console.In.ReadToEnd().AsSpan().Trim();

int part1 = 0, part2 = 0;
Span<int> num = stackalloc int[10];
Span<(int Value, int Length)> arr = stackalloc (int, int)[14];
foreach (var item in span.EnumerateLines())
{
    var enumerator = new SpanSliceEnumerator(item, " |");
    for (int i = 0; enumerator.MoveNext(); i++)
    {
        arr[i] = (ToNumber(enumerator.Current), enumerator.Current.Length);
    }
    num[1] = arr.Find(x => x.Length == 2).Value;
    num[4] = arr.Find(x => x.Length == 4).Value;
    num[7] = arr.Find(x => x.Length == 3).Value;
    num[8] = arr.Find(x => x.Length == 7).Value;
    
    num[6] = arr.Find((x, arg) => x.Length == 6 && (x.Value & arg) != arg, num[1]).Value;
    num[9] = arr.Find((x, arg) => x.Length == 6 && (x.Value & arg) == arg, num[4]).Value;
    num[0] = arr.Find((x, arg) => x.Length == 6 && x.Value != arg.Item1 && x.Value != arg.Item2, (num[9], num[6])).Value;

    num[3] = arr.Find((x, arg) => x.Length == 5 && (x.Value & arg) == arg, num[1]).Value;
    num[5] = arr.Find((x, arg) => x.Length == 5 && x.Value != arg.Item1 && (x.Value & arg.Item2) != 0, (num[3], num[6] & num[1])).Value;
    num[2] = arr.Find((x, arg) => x.Length == 5 && x.Value != arg.Item1 && x.Value != arg.Item2, (num[3], num[5])).Value;

    int temp = 0;
    foreach (var itemNum in arr.Slice(10))
    {
        temp = temp * 10 + num.IndexOf(itemNum.Value);
        if (itemNum.Length is 2 or 3 or 4 or 7)
            part1++;
    }
    part2 += temp;
}
Console.WriteLine(part1);
Console.WriteLine(part2);
