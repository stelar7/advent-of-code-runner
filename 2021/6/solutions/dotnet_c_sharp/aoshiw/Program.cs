
        var span = Console.In.ReadToEnd().AsSpan().Trim();
        Span<ulong> fish = stackalloc ulong[9];
        foreach (var item in new SpanSliceEnumerator(span, ","))
        {
            fish[int.Parse(item)]++;
        }
        int i = 0;
        ulong temp, part1 = 0, part2 = 0;
        while (i++ < 80)
        {
            temp = fish[0];
            for (int ii = 0; ii < fish.Length - 1; ii++)
            {
                fish[ii] = fish[ii + 1];
            }
            fish[6] += temp;
            fish[8] = temp;
        }
        foreach (var item in fish)
        {
            part1 += item;
        }
        Console.WriteLine(part1);
        --i;
        while (i++ < 256)
        {
            temp = fish[0];
            for (int ii = 0; ii < fish.Length - 1; ii++)
            {
                fish[ii] = fish[ii + 1];
            }
            fish[6] += temp;
            fish[8] = temp;
        }
        foreach (var item in fish)
        {
            part2 += item;
        }
        Console.WriteLine(part2);
