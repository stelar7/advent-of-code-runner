using System;
using System.Collections;
using System.Collections.Generic;
using System.Drawing;


/// <summary>
/// Just 2D Array. because why not?
/// </summary>
/// <typeparam name="T"></typeparam>
public class Map2D<T> : IEnumerable<T>
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
        get => this[point.X, point.Y];
        set => this[point.X, point.Y] = value;
    }

    public Map2D()
    {
        _array = new T[10][];
    }

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
}
