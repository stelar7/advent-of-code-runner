using renkon;

long DoPartA() => ProcessWithMapConfig(true, true, false, 1);

long DoPartB() => ProcessWithMapConfig(true, true, true, 1);

long ProcessWithMapConfig(bool countHorizontals, bool countVerticals, bool countDiagonals, int moreThanValue)
{
    var strs = Utils.InputToStringArray("5");
    var lines = strs.Select(x => x.Split(" -> ")).Select(s => (new Point(s[0]), new Point(s[1])));
    var xSize = lines.Select(line => Math.Max(line.Item1.X, line.Item2.X)).Max() + 1;
    var ySize = lines.Select(line => Math.Max(line.Item1.Y, line.Item2.Y)).Max() + 1;
    var map = new Map(countHorizontals, countVerticals, countDiagonals, xSize, ySize);

    foreach (var line in lines)
    {
        map.AddLine(line.Item1, line.Item2);
    }

    return map.GetPointsWithMoreThanNLines(moreThanValue);
}

Console.WriteLine(DoPartA() + "\n" + DoPartB());

public class Map
{
    private readonly bool countHorizontals;
    private readonly bool countVerticals;
    private readonly bool countDiagonals;
    private readonly int[] mapFields;
    private readonly int width;
    private readonly int height;

    public Map(bool countHorizontals, bool countVerticals, bool countDiagonals, int xSize, int ySize)
    {
        this.countHorizontals = countHorizontals;
        this.countVerticals = countVerticals;
        this.countDiagonals = countDiagonals;
        mapFields = new int[xSize * ySize];
        width = xSize;
        height = ySize;
    }

    public void AddLine(Point from, Point to)
    {
        if (ShouldBeAdded(from, to))
        {
            Add(from.GetPointsBetween(to));
        }
    }

    public int GetPointsWithMoreThanNLines(int numberOfLines)
        => mapFields.Count(i => i > numberOfLines);

    private void Add(IEnumerable<Point> points)
    {
        foreach (var point in points)
        {
            mapFields[GetIndexFromPoint(point)]++;
        }
    }

    private int GetIndexFromPoint(Point point)
        => point.Y * width + point.X;

    private bool ShouldBeAdded(Point from, Point to)
        => (from.IsHorizontallyRelatedTo(to) && countHorizontals) ||
            (from.IsVerticallyRelatedTo(to) && countVerticals) ||
            (from.IsDiagonallyRelatedTo(to) && countDiagonals);
}

public class Point
{
    public int X { get; }
    public int Y { get; }

    public Point(int x, int y)
    {
        X = x;
        Y = y;
    }

    public Point(string str)
    {
        var splitCoords = str.Split(",");
        X = int.Parse(splitCoords[0]);
        Y = int.Parse(splitCoords[1]);
    }

    public IEnumerable<Point> GetPointsBetween(Point otherPoint)
    {
        if (IsHorizontallyRelatedTo(otherPoint))
        {
            return Utils.GetRangeBetween(this.X, otherPoint.X)
                .Select(n => new Point(n, this.Y));
        }
        else if (IsVerticallyRelatedTo(otherPoint))
        {
            return Utils.GetRangeBetween(this.Y, otherPoint.Y)
                .Select(n => new Point(this.X, n));
        }
        else if (IsDiagonallyRelatedTo(otherPoint))
        {
            return Utils.GetRangeBetween(this.X, otherPoint.X)
                .Zip(Utils.GetRangeBetween(this.Y, otherPoint.Y))
                .Select(coords => new Point(coords.First, coords.Second));
        }

        return Enumerable.Empty<Point>();
    }

    public bool IsHorizontallyRelatedTo(Point otherPoint)
        => this.Y == otherPoint.Y;

    public bool IsVerticallyRelatedTo(Point otherPoint)
        => this.X == otherPoint.X;

    public bool IsDiagonallyRelatedTo(Point otherPoint)
        => Math.Abs(this.X - otherPoint.X) == Math.Abs(this.Y - otherPoint.Y);
}