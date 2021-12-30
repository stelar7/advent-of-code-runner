record struct Pawn
{
    public Pawn(int space)
    {
        Space = space;
    }
    public int Space;
    public int Score = 0;
    public static Pawn operator +(Pawn p, int i)
    {
        p.Space += i;
        if (p.Space > 10)
            p.Space %= 10;
        p.Score += p.Space == 0 ? 10 : p.Space;
        return p;
    }
}
