var span = Console.In.ReadToEnd().AsSpan().Trim();
Span<int> maxCalories = stackalloc int[3];
var calories = 0;
foreach (var item in span.EnumerateLines())
{
    if (item.IsEmpty)
    {
        if (calories > maxCalories[0])
        {
            maxCalories[0] = calories;
            maxCalories.Sort();
        }
        calories = 0;
    }
    else
    {
        calories += int.Parse(item);
    }
}
Console.WriteLine(maxCalories[2]);
Console.WriteLine(maxCalories[2]+ maxCalories[1]+ maxCalories[0]);
