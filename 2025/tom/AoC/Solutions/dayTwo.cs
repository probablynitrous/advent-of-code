using AoC.Helpers;

namespace AoC.Solutions;

public class DayTwo
{
    static public async Task<string> Run(bool isTest = false)
    {
        string[] contents = await FileHelper.ReadFile($"AoC.inputs._02.{(isTest ? "test" : "main")}.txt");
        var ranges = contents[0].Split(',').Select(r =>
        {
            var parts = r.Split('-');
            return (long.Parse(parts[0]), long.Parse(parts[1]));
        }).ToArray();

        long invalidTotal1 = 0;
        long invalidTotal2 = 0;
        foreach (var (start, end) in ranges)
        {
            for (long i = start; i <= end; i++)
            {
                if (IsInvalidPart1(i))
                {
                    invalidTotal1 += i;
                }
                if (IsInvalidPart2(i))
                {
                    invalidTotal2 += i;
                }
            }
        }
        return $"Invalid total: {invalidTotal1}, {invalidTotal2}";
    }

    static bool IsInvalidPart1(long num)
    {
        string numStr = num.ToString();
        int length = numStr.Length;
        if (length % 2 != 0)
        {
            return false;
        }

        int mid = length / 2;
        var left = numStr[..mid];
        var right = numStr[mid..];
        return left == right;
    }

    static bool IsInvalidPart2(long num)
    {
        string numStr = num.ToString();
        int length = numStr.Length;

        for (int i = 1; i <= length / 2; i++)
        {
            if (length % i == 0)
            {
                string pattern = numStr[..i];
                int potentialMatches = length / i;

                string constructed = string.Concat(Enumerable.Repeat(pattern, potentialMatches));

                if (constructed == numStr)
                {
                    return true;
                }
            }
        }
        return false;
    }
}