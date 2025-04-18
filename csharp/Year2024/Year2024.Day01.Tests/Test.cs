namespace Year2024.Day01.Tests;

[TestClass]
public class ChallengeTests
{
    private readonly Year2024.Day01.Challenge challenge;
    private readonly string inputDir;

    public ChallengeTests()
    {
        string[] typeName = this.GetType().ToString().Split('.');
        string year = typeName[0].Substring("Year".Length);
        string day = typeName[1].Substring("Day".Length);
        challenge = new Year2024.Day01.Challenge();
        inputDir = $"../../../../../../rsrc/inputs/year_{year}/day_{day}/tests";
    }

    [TestMethod]
    [DataRow("practice_01.txt", 11UL)]
    [DataRow("final.txt", 1530215UL)]
    public void Part01(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile)).Trim();
        Assert.AreEqual(expected, challenge.Part01(input));
    }

    [TestMethod]
    [DataRow("practice_01.txt", 31UL)]
    [DataRow("final.txt", 26800609UL)]
    public void Part02(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile)).Trim();
        Assert.AreEqual(expected, challenge.Part02(input));
    }
}
