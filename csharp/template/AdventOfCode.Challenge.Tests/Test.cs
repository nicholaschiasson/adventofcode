namespace AdventOfCode.Challenge.Tests;

[TestClass]
public class ChallengeTests
{
    private readonly AdventOfCode.Challenge.Challenge challenge;
    private readonly string inputDir;

    public ChallengeTests()
    {
        string[] typeName = this.GetType().ToString().Split('.');
        string year = typeName[0].Substring("Year".Length);
        string day = typeName[1].Substring("Day".Length);
        challenge = new AdventOfCode.Challenge.Challenge();
        inputDir = $"../../../../../../rsrc/inputs/year_{year}/day_{day}/tests";
    }

    [TestMethod]
    [DataRow("practice_1.txt", 0UL)]
    [DataRow("final.txt", 0UL)]
    public void Part01(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile)).Trim();
        Assert.AreEqual<ulong>(expected, challenge.Part01(input));
    }

    [TestMethod]
    [DataRow("practice_1.txt", 0UL)]
    [DataRow("final.txt", 0UL)]
    public void Part02(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile)).Trim();
        Assert.AreEqual<ulong>(expected, challenge.Part02(input));
    }
}
