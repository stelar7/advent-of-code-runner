int x = 0, y = 0, z = 0;
string s;
while ((s = Console.ReadLine()) != null)
{
    switch (s[0])
    {
        case 'f':
            var c = s[8] - '0';
            x += c;
            z += y * c;
            break;
        case 'd':
            y += s[5] - '0';
            break;
        case 'u':
            y -= s[3] - '0';
            break;
    }
}

Console.WriteLine(x * y + "\n" + x * z);
