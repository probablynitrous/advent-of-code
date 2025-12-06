using AoC.Helpers;

namespace AoC.Solutions;

public class DayFour
{
    static public async Task<string> Run(bool isTest = false)
    {
        string[] contents = await FileHelper.ReadFile($"AoC.inputs._04.{(isTest ? "test" : "main")}.txt");
        
        var grid = BuildGrid(contents);
        int solution1 = Solution1(grid);
        int solution2 = Solution2(grid);
        return $"Solution 1: {solution1}\nSolution 2: {solution2}";
    }   

    static int Solution1(Dictionary<(int x, int y), string> grid)
    {
        int validPositions = 0;
        foreach (var key in grid.Keys.ToList())
        {
            if (grid[key] == ".")
                continue;
            int adjacentSum = FindAdjacentSum(grid, key);
            if (adjacentSum < 4)
            {
                validPositions++;
            }
        }
        return validPositions;
    }

    static int Solution2(Dictionary<(int x, int y), string> grid)
    {
        bool valid = true;
        var newGrid = grid;
        int validPositions = 0;
        while (valid)
        {
            grid = newGrid;
            valid = false;
            foreach (var key in grid.Keys.ToList())
            {
                if (grid[key] == ".")
                    continue;
                int adjacentSum = FindAdjacentSum(grid, key);
                if (adjacentSum < 4)
                {
                    validPositions++;
                    newGrid[key] = ".";
                    valid = true;
                }
            }

        }
        return validPositions;
    }

    static Dictionary<(int x, int y), string> BuildGrid(string[] contents)
    {
        var grid = new Dictionary<(int x, int y), string>();
        for (int y = 0; y < contents.Length; y++)
        {
            var line = contents[y];
            for (int x = 0; x < line.Length; x++)
            {
                grid[(x, y)] = line[x].ToString();
            }
        }
        return grid;
    }

    static int FindAdjacentSum(Dictionary<(int x, int y), string> grid, (int x, int y) key)
    {
        (int x, int y)[] adjacentCoords = GetAdjacentCoords(key);
        int sum = 0;
        foreach (var (adjX, adjY) in adjacentCoords)
        {
            if (grid.ContainsKey((adjX, adjY)))
            {
                string value = grid[(adjX, adjY)];
                if (value == "@")
                {
                    sum++;
                }
            }
        }
        return sum;
    }

    static (int x, int y)[] GetAdjacentCoords((int x, int y) key)
    {
        return
        [
            (key.x - 1, key.y),
            (key.x + 1, key.y),
            (key.x, key.y - 1),
            (key.x, key.y + 1),
            (key.x - 1, key.y - 1),
            (key.x + 1, key.y - 1),
            (key.x - 1, key.y + 1),
            (key.x + 1, key.y + 1)
        ];
    }
}