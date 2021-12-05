using renkon;
using System.Text;

long DoPartA()
{
    var numbers = Utils.InputToStringArray("3");
    var gammaBuilder = new StringBuilder();
    var epsilonBuilder = new StringBuilder();

    foreach (var mapper in GetCountMappers(numbers))
    {
        gammaBuilder.Append(mapper.MaxBy(kv => kv.Count).Key);
        epsilonBuilder.Append(mapper.MinBy(kv => kv.Count).Key);
    }

    return Convert.ToInt32(gammaBuilder.ToString(), 2) * Convert.ToInt32(epsilonBuilder.ToString(), 2);
}

long DoPartB()
{
    var numbers = Utils.InputToStringArray("3");

    return Convert.ToInt32(ProcessAgregation(numbers, n => n.MaxBy(e => e.Count), ie => ie.OrderByDescending(e => e.Key)), 2)
        * Convert.ToInt32(ProcessAgregation(numbers, n => n.MinBy(e => e.Count), ie => ie.OrderBy(e => e.Key)), 2);
}

IEnumerable<IEnumerable<(char Key, int Count)>> GetCountMappers(IEnumerable<string> numbers)
    => Enumerable.Range(0, numbers.First().Length)
        .Select(i => numbers.GroupBy(n => n[i])
            .Select(g => (g.Key, g.Count())));

IEnumerable<IEnumerable<(char Key, int Count)>> GetCountMappers2(
    IEnumerable<string> numbers,
    Func<IEnumerable<(char Key, int Count)>, IEnumerable<(char Key, int Count)>> orderingFn)
    => GetCountMappers(numbers).Select(n => orderingFn.Invoke(n));

string ProcessAgregation(
    IEnumerable<string> numbers,
    Func<IEnumerable<(char Key, int Count)>, (char Key, int Count)> aggregationCauseFn,
    Func<IEnumerable<(char Key, int Count)>, IEnumerable<(char Key, int Count)>> orderingFn)
    => Enumerable.Range(0, numbers.First().Length)
        .Aggregate(
            numbers,
            (nums, i) =>
            {
                var maxKey = aggregationCauseFn.Invoke(GetCountMappers2(nums, orderingFn).ElementAt(i)).Key;
                return nums.Where(n => n[i] == maxKey);
            })
        .Single();

Console.WriteLine(DoPartA() + "\n" + DoPartB());