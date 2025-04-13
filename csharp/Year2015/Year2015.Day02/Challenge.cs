namespace Year2015.Day02;

public class Challenge
{
    public ulong Part01(string input)
    {
        return input
            .Split('\n', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries)
            .Select(x => x
                .Split('x', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries)
                .Select(x => Convert.ToUInt64(x))
                .ToArray()
            )
            .Select(x =>
            {
                ulong area = 0;
                var minArea = ulong.MaxValue;
                for (var i = 0; i < x.Length - 1; i++)
                {
                    for (var j = i + 1; j < x.Length; j++)
                    {
                        var a = x[i] * x[j];
                        minArea = Math.Min(minArea, a);
                        area += a * 2;
                    }
                }

                return area + minArea;
            })
            .Aggregate((a, b) => a + b);
    }

    public ulong Part02(string input)
    {
        return input
            .Split('\n', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries)
            .Select(x => x
                .Split('x', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries)
                .Select(x => Convert.ToUInt64(x))
                .ToArray()
            )
            .Select(x =>
            {
                var minPerimeter = ulong.MaxValue;
                for (var i = 0; i < x.Length - 1; i++)
                {
                    for (var j = i + 1; j < x.Length; j++)
                    {
                        minPerimeter = Math.Min(minPerimeter, x[i] * 2 + x[j] * 2);
                    }
                }

                return x.Aggregate((a, b) => a * b) + minPerimeter;
            })
            .Aggregate((a, b) => a + b);
    }
}