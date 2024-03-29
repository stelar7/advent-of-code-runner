﻿using System.Drawing;

public class Map2D<T>
{
    T[][] _array;
    public int Row { get; private set; }
    public int Column => _array[0].Length;

    public T this[int row, int column]
    {
        get => _array[row][column];
        set => _array[row][column] = value;
    }

    public T this[Point point]
    {
        get => this[point.Y, point.X];
        set => this[point.Y, point.X] = value;
    }

    public Map2D()
    {
        _array = new T[10][];
    }

    public bool IsInRange(Point point) => point.X.IsInRange(0, Column) && point.Y.IsInRange(0, Row);

    public void AddRow(ReadOnlySpan<T> row) => AddRow(row, x => x);

    public void AddRow<TIn>(ReadOnlySpan<TIn> row, Func<TIn, T> func)
    {
        if (Row != 0 && Column != row.Length)
            throw new ArgumentOutOfRangeException(nameof(row));
        if (_array.Length == Row)
        {
            Array.Resize(ref _array, Row + 10);
        }
        _array[Row] = new T[row.Length];
        for (int i = 0; i < row.Length; i++)
        {
            _array[Row][i] = func(row[i]);
        }
        Row++;
    }

    public IEnumerable<Point> GetPointEnumerator()
    {
        var p = new Point();
        for (; p.Y < Row; p.Y++)
        {
            for (p.X = 0; p.X < Column; p.X++)
            {
                yield return p;
            }
        }
    }
}
