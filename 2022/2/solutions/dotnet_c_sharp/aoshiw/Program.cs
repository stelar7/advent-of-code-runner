var span = Console.In.ReadToEnd().AsSpan().TrimEnd();
int point1 = 0, point2 = 0;
var map1 = new int[,]
{
    { 1+3, 2+6, 3+0 },
    { 1+0, 2+3, 3+6 },
    { 1+6, 2+0, 3+3 }
};   
var map2 = new int[,]
{  
    { 3+0, 1+3, 2+6 },
    { 1+0, 2+3, 3+6 },
    { 2+0, 3+3, 1+6 }
};

foreach (var item in span.EnumerateLines())
{
    point1 += map1[item[0] - 'A', item[2] - 'X'];
    point2 += map2[item[0] - 'A', item[2] - 'X'];
}
Console.WriteLine(point1);
Console.WriteLine(point2);
