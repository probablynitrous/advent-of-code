using AoC.Helpers;

namespace AoC;

public class DayOne
{
    static public async Task<string> Run(bool isTest = false)
    {
        string[] contents = await FileHelper.ReadFile($"AoC.inputs._01.{(isTest ? "test" : "main")}.txt");
        int currPos = 50;
        int zeroCounter = 0;
        int resetCounter = 0;
        foreach (var line in contents)
        {
            int change = line[0] == 'L'
                ? -int.Parse(line[1..])
                : int.Parse(line[1..]);

            resetCounter += CountResets(currPos, change);

            currPos = ((currPos + change) % 100 + 100) % 100;

            if (currPos == 0)
                zeroCounter++;
        }
        return "Zeros hit: " + zeroCounter + ", Resets: " + resetCounter;
    }
    private static int CountResets(int currPos, int change)
    {
        int steps = Math.Abs(change);
        int dir = change > 0 ? 1 : -1;
        int resets = steps / 100;
        int remainder = steps % 100;
        int pos = currPos;
        for (int i = 0; i < remainder; i++)
        {
            pos = (pos + dir + 100) % 100;
            if (pos == 0)
                resets++;
        }
        return resets;
    }

}