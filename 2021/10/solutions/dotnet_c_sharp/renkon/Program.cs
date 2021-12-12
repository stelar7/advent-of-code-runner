using renkon;

long DoPartA()
{
    var entries = Utils.InputToStringArray("10").Where(s => !string.IsNullOrEmpty(s));
    var pointsCounter = 0L;

    foreach (var str in entries)
    {
        var stack = new Stack<char>();

        foreach (var c in str)
        {
            if (isOpener(c))
            {
                stack.Push(c);
            }
            else if (GetReverseCharacter(c) != stack.Pop())
            {
                pointsCounter += GetPointsForNotClosedCharacter(c);
                break;
            }
        }
    }

    return pointsCounter;
}

long DoPartB()
{
    var entries = Utils.InputToStringArray("10").Where(s => !string.IsNullOrEmpty(s));
    var pointsList = new List<long>();

    foreach (var str in entries)
    {
        var stack = new Stack<char>();
        var error = false;

        foreach (var c in str)
        {
            if (isOpener(c))
            {
                stack.Push(c);
            }
            else if (GetReverseCharacter(c) != stack.Pop())
            {
                error = true;
                break;
            }
        }

        if (!error && stack.Count() > 0)
        {
            var pointsCounter = 0L;

            while (stack.Count() != 0)
            {
                pointsCounter = pointsCounter * 5 + GetPointsForWrongClosingCharacter(stack.Pop());
            }

            pointsList.Add(pointsCounter);
        }
    }

    return pointsList.OrderBy(x => x).ElementAt(pointsList.Count() / 2);
}

bool isOpener(char c)
    => new[] { '(', '[', '{', '<' }.Contains(c);

char GetReverseCharacter(char c)
{
    switch (c)
    {
        case ')':
            return (char)(c - 1);
        default:
            return (char)(c - 2);
    }
}

long GetPointsForWrongClosingCharacter(char c)
{
    switch (c)
    {
        case '(':
            return 1L;
        case '[':
            return 2L;
        case '{':
            return 3L;
        case '<':
            return 4L;
    }

    return -1;
}

long GetPointsForNotClosedCharacter(char c)
{
    switch (c)
    {
        case ')':
            return 3;
        case ']':
            return 57;
        case '}':
            return 1197;
        case '>':
            return 25137;
    }

    return -1;
}

Console.WriteLine(DoPartA() + "\n" + DoPartB());