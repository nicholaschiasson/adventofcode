namespace Year2024.Day01;

public class Challenge
{
    public ulong Part01(string input)
    {
        var lines = 0;
        var left = new List<int>();
        var right = new List<int>();
        foreach (var line in input.Split('\n'))
        {
            var split = line.Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
            lines++;
            left.Add(int.Parse(split[0]));
            right.Add(int.Parse(split[1]));
        }

        left.Sort();
        right.Sort();

        var totalDistance = 0ul;
        for (var i = 0; i < lines; i++)
        {
            totalDistance += (ulong)Math.Abs(left[i] - right[i]);
        }

        return totalDistance;
    }

    public ulong Part02(string input)
    {
        var lines = 0;
        var left = new List<int>();
        var right = new List<int>();
        foreach (var line in input.Split('\n'))
        {
            var split = line.Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
            lines++;
            left.Add(int.Parse(split[0]));
            right.Add(int.Parse(split[1]));
        }

        var occurrences = new Dictionary<int, ulong>();
        var similartyScore = 0ul;
        foreach (var n in left)
        {
            if (occurrences.ContainsKey(n))
            {
                similartyScore += occurrences[n];
            }
            else
            {
                var count = 0ul;
                foreach (var o in right)
                {
                    if (n == o)
                    {
                        count += 1;
                    }
                }
                count *= (ulong)n;
                occurrences[n] = count;
                similartyScore += count;
            }
        }
        return similartyScore;
    }
}
