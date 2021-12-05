using renkon;

int DoPartA()
{
    return CountIncreasesByStep(GetListOfIntegers(), 1);
}

int DoPartB()
{
    return CountIncreasesByStep(GetListOfIntegers(), 3);
}

int CountIncreasesByStep(IList<int> numbers, int stepSize)
{
    return numbers.Take(new Range(0, numbers.Count - stepSize))
        .Select((n, i) => n - numbers[i + stepSize])
        .Count(n => n < 0);
}

IList<int> GetListOfIntegers()
{
    return Utils.StringArrayToIntArray(Utils.InputToStringArray("1")).ToList();
}

Console.WriteLine(DoPartA() + "\n" + DoPartB());