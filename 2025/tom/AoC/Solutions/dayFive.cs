using AoC.Helpers;

namespace AoC.Solutions;

public class DayFive
{
    static public async Task<string> Run(bool isTest = false)
    {
        string[] contents = await FileHelper.ReadFile($"AoC.inputs._05.{(isTest ? "test" : "main")}.txt");

        int splitIndex = contents.IndexOf("");

        string[] freshRanges = contents[..splitIndex];
        string[] ingredientIds = contents[(splitIndex + 1)..];

        int freshIngedients = 0;

        long[] freshIds = [];

        (long start, long end)[] freshRangesParsed = [.. freshRanges
            .Select(r =>
            {
                string[] parts = r.Split('-');
                return (long.Parse(parts[0]), long.Parse(parts[1]));
            })];

        var sortedRanges = freshRangesParsed.OrderBy(r => r.start).ToList();
        (long start, long end)[] merged = [sortedRanges[0]];
        
        foreach (var range in sortedRanges)
        {
            var last = merged[^1];

            if (range.start <= last.end + 1)
            {
                merged[^1] = (last.start, Math.Max(last.end, range.end));
            }
            else
            {
                merged = [.. merged, range];
            }
        }

        foreach (var id in ingredientIds)
        {
            if (id == "") continue;
            long ingredientId = long.Parse(id);
            foreach (var (start, end) in merged)
            {                
                if (ingredientId >= start && ingredientId <= end)
                {
                    freshIngedients++;
                    break;
                }
            }
        }

        long total = 0;
        foreach (var (start, end) in merged)
        {
            total += end - start + 1;
        }
        return $"{freshIngedients} fresh ingredients found with a total of {total} possible IDs.";
    }
}