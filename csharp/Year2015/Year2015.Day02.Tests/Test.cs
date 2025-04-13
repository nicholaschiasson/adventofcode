namespace Year2015.Day02.Tests;

[TestClass]
public class ChallengeTests
{
    private readonly Year2015.Day02.Challenge challenge;
    private readonly string inputDir;

    public ChallengeTests()
    {
        string[] typeName = this.GetType().ToString().Split('.');
        string year = typeName[0].Substring("Year".Length);
        string day = typeName[1].Substring("Day".Length);
        challenge = new Year2015.Day02.Challenge();
        inputDir = $"../../../../../../rsrc/inputs/year_{year}/day_{day}/tests";
    }

    [TestMethod]
    [DataRow("practice_1.txt", 58UL)]
    [DataRow("practice_2.txt", 43UL)]
    [DataRow("final.txt", 1588178UL)]
    public void Part01(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile)).Trim();
        Assert.AreEqual<ulong>(expected, challenge.Part01(input));
    }

    [TestMethod]
    [DataRow("practice_1.txt", 34UL)]
    [DataRow("practice_2.txt", 14UL)]
    [DataRow("final.txt", 3783758UL)]
    public void Part02(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile)).Trim();
        Assert.AreEqual<ulong>(expected, challenge.Part02(input));
    }
}
