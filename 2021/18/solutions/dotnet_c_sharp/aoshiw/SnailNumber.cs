using System.Diagnostics.CodeAnalysis;

class SnailNumber
{
    public static SnailNumber Parse(ReadOnlySpan<char> snailNumber)
    {
        if (snailNumber[0] == '[')
        {
            var i = 1;
            for (int b = 0; b != 0 || i == 1; i++)
            {
                if (snailNumber[i] == '[')
                {
                    b++;
                }
                else if (snailNumber[i] == ']')
                {
                    b--;
                }
            }
            var temp = new SnailNumber()
            {
                Left = Parse(snailNumber.Slice(1, i - 1)),
                Right = Parse(snailNumber.Slice(++i, snailNumber.Length - i))
            };
            temp.Left.Parrent = temp.Right.Parrent = temp;
            return temp;
        }
        else
        {
            return new() { Value = snailNumber[0] - '0' };
        }
    }

    [MemberNotNullWhen(false, nameof(Left), nameof(Right))]
    public bool IsValue => Left is null || Right is null;
    public int Value;
    public SnailNumber? Left, Right;
    private SnailNumber? Parrent;

    public int GetMagnitude() => IsValue ? Value : 3 * Left.GetMagnitude() + 2 * Right.GetMagnitude();
    static SnailNumber? Copy(SnailNumber? sn)
    {
        if (sn is null)
            return null;
        var temp = new SnailNumber()
        {
            Value = sn.Value,
            Left = Copy(sn.Left),
            Right = Copy(sn.Right),
        };
        if (!temp.IsValue)
            temp.Left.Parrent = temp.Right.Parrent = temp;
        return temp;
    }

    public static bool TryExplode(SnailNumber? snailNumber, int level)
    {
        if (snailNumber is null || snailNumber.IsValue)
            return false;
        if (level == 4)
        {
            var tempSN = snailNumber;
            while (tempSN == tempSN.Parrent?.Left)
            {
                tempSN = tempSN.Parrent;
            }
            tempSN = tempSN?.Parrent?.Left;
            if (tempSN is not null)
            {
                while (!tempSN.IsValue)
                {
                    tempSN = tempSN.Right;
                }
                tempSN.Value += snailNumber.Left.Value;
            }
            tempSN = snailNumber;
            while (tempSN == tempSN.Parrent?.Right)
            {
                tempSN = tempSN.Parrent;
            }
            tempSN = tempSN?.Parrent?.Right;
            if (tempSN is not null)
            {
                while (!tempSN.IsValue)
                {
                    tempSN = tempSN.Left;
                }
                tempSN.Value += snailNumber.Right.Value;
            }
            snailNumber.Left = snailNumber.Right = null;
            return true;
        }
        level++;
        return TryExplode(snailNumber.Left, level) || TryExplode(snailNumber.Right, level);
    }

    public static bool TrySplit(SnailNumber snailNumber)
    {
        if (snailNumber.IsValue)
        {
            if (snailNumber.Value > 9)
            {
                snailNumber.Left = new()
                {
                    Parrent = snailNumber,
                    Value = snailNumber.Value / 2,
                };
                snailNumber.Right = new()
                {
                    Parrent = snailNumber,
                    Value = snailNumber.Value - snailNumber.Left.Value,
                };
                snailNumber.Value = 0;
                return true;
            }
            return false;
        }
        else
        {
            return TrySplit(snailNumber.Left) || TrySplit(snailNumber.Right);
        }
    }

    public static SnailNumber operator +(SnailNumber left, SnailNumber right)
    {
        var temp = new SnailNumber()
        {
            Left = Copy(left),
            Right = Copy(right)
        };
        temp.Left!.Parrent = temp.Right!.Parrent = temp;
        while (TryExplode(temp, 0) || TrySplit(temp)) ;
        return temp;
    }

    public override string ToString() => IsValue ? Value.ToString() : $"[{Left},{Right}]";
}
