using System;

ref struct SpanSliceEnumerator
{
    ReadOnlySpan<char> _span;
    ReadOnlySpan<char> _slice;

    public SpanSliceEnumerator(ReadOnlySpan<char> span, ReadOnlySpan<char> slice)
    {
        _span = span;
        _slice = slice;
    }

    public SpanSliceEnumerator GetEnumerator()
    {
        return this;
    }

    public ReadOnlySpan<char> Current { get; private set; } = default;

    public bool MoveNext()
    {
        if (_span.Length == 0)
            return false;
        do
        {
            var i = _span.IndexOfAny(_slice);
            if (i == -1)
            {
                Current = _span;
                _span = default;
            }
            else
            {
                Current = _span.Slice(0, i);
                _span = _span.Slice(i + 1);
            }
        } while (Current.Length == 0 && _span.Length != 0);
        return Current.Length != 0;
    }
}
