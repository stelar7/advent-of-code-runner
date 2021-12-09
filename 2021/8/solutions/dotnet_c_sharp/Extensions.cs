static class Extensions
{
    public static T Find<T>(this Span<T> source, Predicate<T> predicate)
    {
        foreach (var item in source)
        {
            if (predicate(item))
                return item;
        }
        throw new ArgumentException();
    }
    public static T Find<T,A>(this Span<T> source, Func<T, A, bool> predicate, A argument)
    {
        foreach (var item in source)
        {
            if (predicate(item, argument))
                return item;
        }
        throw new ArgumentException();
    }
}
