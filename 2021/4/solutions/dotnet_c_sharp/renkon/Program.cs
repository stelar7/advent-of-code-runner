using renkon;

long DoPartA()
{
    var (withdrawnNumbers, boards) = LoadDataFromInput("4");

    foreach (var number in withdrawnNumbers)
    {
        boards.ForEach(board => board.Mark(number));

        if (boards.Any(board => board.HasWon()))
        {
            return boards.First(board => board.HasWon()).Score();
        }
    }

    return -1;
}

long DoPartB()
{
    var (withdrawnNumbers, boards) = LoadDataFromInput("4");
    var lastBoard = default(Board);

    foreach (var number in withdrawnNumbers)
    {
        boards.ForEach(board => board.Mark(number));

        boards.Where(board => board.HasWon())
            .ToList()
            .ForEach(board =>
            {
                boards.Remove(board);
                lastBoard = board;
            });
    }

    return lastBoard!.Score();
}

(IEnumerable<int>, List<Board>) LoadDataFromInput(string inputName)
{
    var input = Utils.InputToStringArray(inputName);
    var inputUnitSize = 6;
    var withdrawnNumbers = input.First().Split(",").Select(int.Parse);

    return (withdrawnNumbers,
        Enumerable.Range(0, (input.Count() - 1) / inputUnitSize)
        .Select(i => i * inputUnitSize + 2)
        .Select(i => new Board(input.Take(new Range(i, i + inputUnitSize - 1))))
        .ToList());
}

Console.WriteLine(DoPartA() + "\n" + DoPartB());
public class BoardEntry
{
    private readonly int number;
    private bool marked;

    public BoardEntry(string number)
    {
        this.number = int.Parse(number);
        this.marked = false;
    }

    public bool IsMarked() => marked;

    public void SetMarked() => marked = true;

    public int GetNumber() => number;
}

public class Board
{
    private readonly int boardSize = 5;
    private int score = -1;
    private readonly List<BoardEntry> boardEntries;

    public Board(IEnumerable<string> input)
    {
        boardEntries = new List<BoardEntry>(
            string.Join(" ", input).Split(" ")
                .Where(s => !string.IsNullOrEmpty(s))
                .Select(nStr => new BoardEntry(nStr)));
    }

    public IEnumerable<BoardEntry> GetRow(int rowNumber)
    {
        var initIndex = rowNumber * boardSize;
        return boardEntries.Take(new Range(initIndex, initIndex + boardSize));
    }

    public IEnumerable<BoardEntry> GetColumn(int columnNumber)
    {
        return boardEntries.Where((_, i) => i % boardSize == columnNumber);
    }

    public bool HasWon()
        => HasAllMarkedBy(GetRow) || HasAllMarkedBy(GetColumn);

    public void Mark(int number)
    {
        boardEntries.FirstOrDefault(be => be.GetNumber() == number)?.SetMarked();

        if (HasWon())
        {
            score = boardEntries.Where(be => !be.IsMarked()).Sum(be => be.GetNumber()) * number;
        }
    }

    public int Score() => score;

    private bool HasAllMarkedBy(Func<int, IEnumerable<BoardEntry>> selector)
        => Enumerable.Range(0, boardSize).Select(selector).Any(group => group.All(eb => eb.IsMarked()));
}