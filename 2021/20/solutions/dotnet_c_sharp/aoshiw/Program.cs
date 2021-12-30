using Kunc.AdventOfCode.Utils;

var span = Console.In.ReadToEnd().AsSpan().Trim();
const int count = 20;
var map = new List<char[]>();
var temp = new List<char[]>();
foreach (var item in span.EnumerateLines(2))
{
    if (map.Count == 0)
    {
        for (int i = 0, l = count + count + item.Length; i < count; i++)
        {
            map.Add(NewArray(l));
            temp.Add(NewArray(l));
        }
    }
    var arr = NewArray(map[0].Length);
    item.CopyTo(arr.AsSpan().Slice(count));
    map.Add(arr);
    temp.Add(NewArray(arr.Length));
}
for (int i = 0, l = map[0].Length; i < count; i++)
{
    map.Add(NewArray(l));
    temp.Add(NewArray(l));
}
static char[] NewArray(int lenght)
{
    var arr = new char[lenght];
    Array.Fill(arr, '.');
    return arr;
}
//https://www.reddit.com/r/adventofcode/comments/rkgmx9/2021_day_20_images_come_to_life/
//span = ".......#...#.##....#.###.######....#.##..##.#....######.###.#......#.##..##.#....######.###.#....##.#...#.......###.#...#..........#.##..##.#....######.###.#....##.#...#.......###.#...#........##.#...#.......###.#...#.......#...............#..................#.##..##.#....######.###.#....##.#...#.......###.#...#........##.#...#.......###.#...#.......#...............#................##.#...#.......###.#...#.......#...............#...............#...............#...............................................";
var c = '.';
for (int i = 0; i < 2; i++)
{
    for (int y = 0; y < map.Count; y++)
    {
        for (int x = 0; x < map[0].Length; x++)
        {
            temp[y][x] = span[Get(map, x, y, c)];
        }
    }
    c = span[(i & 1) == 0 ? 0 : 511];
    Helper.Swap(ref map, ref temp);
}
return map.Sum(x => x.Count(x => x == '#'));
for (int i = 0; i < 48; i++)
{
    for (int y = 0; y < map.Count; y++)
    {
        for (int x = 0; x < map[0].Length; x++)
        {
            temp[y][x] = span[Get(map, x, y, c)];
        }
    }
    c = span[(i & 1) == 0 ? 0 : 511];
    Helper.Swap(ref map, ref temp);
}
return map.Sum(x => x.Count(x => x == '#'));

int Get(List<char[]> map, int x, int y, char c = '.')
{
    var num = 0;
    for (int yy = y - 1; yy <= y + 1; yy++)
    {
        var row = map.ElementAtOrDefault(yy);
        for (int xx = x - 1; xx <= x + 1; xx++)
        {
            var value = row?.ElementAtOrDefault(xx);
            num = (num << 1) | ((value is null or default(char) ? c : value) == '#' ? 1 : 0);
        }
    }
    return num;
}
