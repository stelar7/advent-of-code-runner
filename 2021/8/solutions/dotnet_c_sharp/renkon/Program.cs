using renkon;

long DoPartA()
{
    var entries = Utils.InputToStringArray("8").Select(s => s.Split(" | ")).Select(e => e[1]);
    var count = 0;

    foreach (var str in entries.Select(e => e.Split(" ")).SelectMany(e => e))
    {
        switch (str.Length)
        {
            case 2:
            case 3:
            case 4:
            case 7:
                count++;
                break;
        }
    }

    return count;
}

long DoPartB()
{
    var inputs = Utils.InputToStringArray("8").Select(s => s.Split(" | ")).Select(e => (e[0], e[1]));

    var chars = "abcdefg";
    var totalSum = 0L;
    var mappings = new Dictionary<string, int>
    {
        ["abcefg"] = 0,
        ["cf"] = 1,
        ["acdeg"] = 2,
        ["acdfg"] = 3,
        ["bcdf"] = 4,
        ["abdfg"] = 5,
        ["abdefg"] = 6,
        ["acf"] = 7,
        ["abcdefg"] = 8,
        ["abcdfg"] = 9
    };
    var permutations = GetPermutations(chars, chars.Length);

    foreach (var input in inputs)
    {
        var inputDigits = input.Item1.Split(" ").Select(s => s.ToCharArray());
        var outputDigits = input.Item2.Split(" ").Select(s => s.ToCharArray());

        foreach (var permutation in permutations)
        {
            var matched = true;
            var permutationStr = new string(permutation.ToArray());

            foreach (var digit in inputDigits.Concat(outputDigits))
            {
                var possibleDigit = digit
                    .Select(d => (char)('a' + permutationStr.IndexOf(d)))
                    .OrderBy(c => c)
                    .ToArray();

                if (!mappings.ContainsKey(new string(possibleDigit)))
                {
                    matched = false;
                    break;
                }
            }

            if (matched)
            {
                var numberValue = 0;
                for (int z = 0; z < 4; z++)
                {
                    numberValue = numberValue +
                        mappings
                        [
                            new string(
                                outputDigits.ElementAt(z)
                                .Select(d => (char)('a' + permutationStr.IndexOf(d)))
                                .OrderBy(c => c)
                                .ToArray())
                        ] * (int)Math.Pow(10, 3 - z);
                }

                totalSum += numberValue;
                break;
            }
        }
    }

    return totalSum;
}

IEnumerable<IEnumerable<T>> GetPermutations<T>(IEnumerable<T> list, int length)
{
    if (length == 1) return list.Select(t => new T[] { t });

    return GetPermutations(list, length - 1)
        .SelectMany(t => list.Where(e => !t.Contains(e)),
            (t1, t2) => t1.Concat(new T[] { t2 }));
}

Console.WriteLine(DoPartA() + "\n" + DoPartB());