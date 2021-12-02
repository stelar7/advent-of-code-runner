List<int> n = new List<int>(2000);
int c1 = 0, c2 = 0;
int i = 0;
string s;
while ((s = Console.ReadLine()) != null)
{
    n.Add(0);

    for (int j = 0; j < s.Length; j++)
    {
        n[i] = n[i] * 10 + (s[j] - '0');
    }

    if (i >= 3)
    {
        if (n[i - 3] < n[i])
        {
            c2++;
        }

        if (n[i - 1] < n[i])
        {
            c1++;
        }
    }
    else if (i >= 1 && n[i - 1] < n[i])
    {
        c1++;
    }

    i++;
}

Console.WriteLine(c1);
Console.WriteLine(c2);