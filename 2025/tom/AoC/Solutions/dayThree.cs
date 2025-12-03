using AoC.Helpers;

namespace AoC.Solutions;

public class DayThree
{
    static public async Task<string> Run(bool isTest = false)
    {
        string[] contents = await FileHelper.ReadFile($"AoC.inputs._03.{(isTest ? "test" : "main")}.txt");
        int solution1 = 0;
        long solution2 = 0;
        foreach (var line in contents)
        {
            if (line == "") continue;
            int[] intArray = line.Select(c => int.Parse(c.ToString())).ToArray();

            solution1 += Solve1(intArray);
            solution2 += Solve2(intArray);
        }
        return $"Solution 1: {solution1}, Solution 2: {solution2}";
    }

    static int Solve1(int[] intArray)
    {
        int first = intArray[..(intArray.Length - 1)].Max();
        int second = intArray[(intArray.IndexOf(first) + 1)..].Max();
        return int.Parse($"{first}{second}");
    }

    static long Solve2(int[] intArray)
    {
        int maxLen = 12;
        int[] solution = [];
        int startIndex = 0;

        while (solution.Length < maxLen)
        {
            int remainingNum = maxLen - solution.Length;

            int searchStart = startIndex;
            int searchEnd = intArray.Length - remainingNum;

            int highestNum = -1;
            int highestIndex = -1;

            for (int i = searchStart; i <= searchEnd; i++)
            {
                if (intArray[i] > highestNum)
                {
                    highestNum = intArray[i];
                    highestIndex = i;

                    if (highestNum == 9)
                        break;
                }
            }

            solution = [.. solution, highestNum];

            startIndex = highestIndex + 1;
        }

        return long.Parse(string.Concat(solution));
    }
}