namespace Year2015.Day01.Tests;

[TestClass]
public class ChallengeTests
{
    private readonly Year2015.Day01.Challenge challenge;
    private readonly string inputDir;

    public ChallengeTests()
    {
        string[] typeName = this.GetType().ToString().Split('.');
        string year = typeName[0].Substring("Year".Length);
        string day = typeName[1].Substring("Day".Length);
        challenge = new Year2015.Day01.Challenge();
        inputDir = $"../../../../../../rsrc/inputs/year_{year}/day_{day}/tests";
    }

    [TestMethod]
    [DataRow("practice_1.txt", 0L)]
    [DataRow("practice_2.txt", 0L)]
    [DataRow("practice_3.txt", 3L)]
    [DataRow("practice_4.txt", 3L)]
    [DataRow("practice_5.txt", 3L)]
    [DataRow("practice_6.txt", -1L)]
    [DataRow("practice_7.txt", -1L)]
    [DataRow("practice_8.txt", -3L)]
    [DataRow("practice_9.txt", -3L)]
    [DataRow("final.txt", 280L)]
    public void Part01(string inputFile, long expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile)).Trim();
        Assert.AreEqual(expected, challenge.Part01(input));
    }

    [TestMethod]
    [DataRow("practice_10.txt", 1UL)]
    [DataRow("practice_11.txt", 5UL)]
    [DataRow("final.txt", 1797UL)]
    public void Part02(string inputFile, ulong expected)
    {
        string input = File.ReadAllText(Path.Join(inputDir, inputFile)).Trim();
        Assert.AreEqual(expected, challenge.Part02(input));
    }
}
