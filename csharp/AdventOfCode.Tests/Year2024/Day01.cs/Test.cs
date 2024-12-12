namespace AdventOfCode.Tests.Year2024;

[TestClass]
public class Day01
{
    private readonly AdventOfCode.Year2024.Day01 challenge;
    private readonly string inputDir;

    public Day01()
    {
        string[] typeName = this.GetType().ToString().Split('.');
        string year = typeName[2].Substring("Year".Length);
        string day = typeName[3].Substring("Day".Length);
        challenge = new AdventOfCode.Year2024.Day01();
        inputDir = $"../../../../../rsrc/inputs/year_{year}/day_{day}/tests";
    }

    [TestMethod]
    [DataRow("practice_01.txt", 11UL)]
    [DataRow("final.txt", 1530215UL)]
    public void Part01(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile));
        Assert.AreEqual<ulong>(expected, challenge.Part01(input));
    }

    [TestMethod]
    [DataRow("practice_01.txt", 31UL)]
    [DataRow("final.txt", 26800609UL)]
    public void Part02(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile));
        Assert.AreEqual<ulong>(expected, challenge.Part02(input));
    }
}
