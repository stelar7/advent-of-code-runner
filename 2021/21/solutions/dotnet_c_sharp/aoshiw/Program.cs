class Program
{
    static (int, int Value)[] Roll = new[] { (1, 3), (1, 9), (3, 4), (3, 8), (6, 5), (6, 7), (7, 6), };
   
    static void Main()
    {
        var span = Console.In.ReadToEnd().AsSpan().Trim();
        var enumerator = span.EnumerateLines();
        enumerator.MoveNext();
        var lastIndex = enumerator.Current.LastIndexOf(' ');
        var p1 = new Pawn(int.Parse(enumerator.Current.Slice(lastIndex + 1)));
        enumerator.MoveNext();
        var p2 = new Pawn(int.Parse(enumerator.Current.Slice(lastIndex + 1)));

        var part2Result = QuantumRoll(p2, p1);
        var part2 = Math.Max(part2Result.Item1, part2Result.Item2);

        var isPlayer1 = true;
        int num = 1;
        int ii = 0;
        while (p1.Score < 1000 && p2.Score < 1000)
        {
            ref var p = ref isPlayer1 ? ref p1 : ref p2;
            var move = num++; if (num == 101) num = 1;
            move += num++; if (num == 101) num = 1;
            move += num++; if (num == 101) num = 1;
            ii += 3;
            p += move;
            isPlayer1 = !isPlayer1;
        }
        Console.WriteLine(ii * Math.Min(p1.Score, p2.Score));
        Console.WriteLine(part2);
    }

    static (long, long) QuantumRoll(Pawn p1, Pawn p2, bool isPlayer1 = true)
    {
        if (p1.Score >= 21)
            return isPlayer1 ? (1, 0) : (0, 1);
        isPlayer1 = !isPlayer1;
        long r1 = 0, r2 = 0;
        foreach (var item in Roll)
        {
            var temp = QuantumRoll(p2 + item.Value, p1, isPlayer1);
            r1 += temp.Item1 * item.Item1;
            r2 += temp.Item2 * item.Item1;
        }
        return (r1, r2);
    }
}
