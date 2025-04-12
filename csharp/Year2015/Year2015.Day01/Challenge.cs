namespace Year2015.Day01;

public class Challenge
{
    public long Part01(string input)
    {
        long floor = 0;
        foreach (var c in input.ToCharArray())
        {
            switch (c) {
                case '(':
                    floor++;
                    break;
                case ')':
                    floor--;
                    break;
                default:
                    throw new InvalidDataException($"Ivalid instruction '{c}'");
            }
        }
        return floor;
    }

    public ulong Part02(string input)
    {
        long floor = 0;
        ulong position = 0;
        foreach (var c in input.ToCharArray())
        {
            position++;
            switch (c) {
                case '(':
                    floor++;
                    break;
                case ')':
                    floor--;
                    break;
                default:
                    throw new InvalidDataException($"Ivalid instruction '{c}'");
            }
            if (floor < 0) {
                break;
            }
        }
        return position;
    }
}
