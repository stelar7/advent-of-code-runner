static class Extensions
{ 
    public static bool IsInRange(this int num, int minInclusive, int maxExclusive)
        => num >= minInclusive && num < maxExclusive;

}
