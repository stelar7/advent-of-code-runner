var input = Console.In.ReadToEnd().AsSpan().Trim();
var num = new List<int>();
foreach (var item in input.EnumerateLines())
{
    num.Add(int.Parse(item));
}

int count1 = 0, count2 = 0;
var sum = num[0] + num[1] + num[2];
for (int i = 1; i < num.Count-2; i++)
{
    if (num[i] > num[i - 1])
        count1++;
    var item = num[i] + num[i + 1] + num[i + 2];
    if (item > sum)
        count2++;
    sum = item;
}
for (int i = num.Count-2; i < num.Count; i++)
{
    if (num[i] > num[i - 1])
        count1++;
}

Console.WriteLine(count1);
Console.WriteLine(count2);
