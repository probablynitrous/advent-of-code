using AoC.Helpers;

namespace AoC.Solutions;

public class DaySix
{
    static public async Task<string> Run(bool isTest = false)
    {
        string[] contents = await FileHelper.ReadFile($"AoC.inputs._06.{(isTest ? "test" : "main")}.txt");
        long solution1 = Solution1(contents);
        long solution2 = Solution2(contents);
        return $"Solution 1: {solution1}\nSolution 2: {solution2}";
    }
    static long Solution1(string[] contents)
    {
        var operations = ParseOperations(contents[^1]);

        Dictionary<int, (string symbol, int[] val)> columns = new Dictionary<int, (string, int[])>();

        foreach (var line in contents[..^1])
        {
            int currPos = 0;
            foreach (int key in operations.Keys)
            {
                var (symbol, width) = operations[key];
                string segment = line.Substring(currPos, width);
                currPos += width + 1;
                if (columns.ContainsKey(key))
                {
                    var (_, val) = columns[key];
                    columns[key] = (symbol, [.. val, int.Parse(segment)]);
                }
                else
                    columns.Add(key, (symbol, [int.Parse(segment)]));
            }
        }

        return CalcSolution(columns, operations);
    }
    static long Solution2(string[] contents)
    {
        var operations = ParseOperations(contents[^1]);

        Dictionary<int, (string symbol, int[] val)> columns = new Dictionary<int, (string, int[])>();

        foreach (var line in contents[..^1])
        {
            int currPos = 0;
            foreach (int key in operations.Keys)
            {
                var (symbol, width) = operations[key];
                if (!columns.TryGetValue(key, out (string symbol, int[] val) value))
                {
                    value = (symbol, new int[width]);
                    columns.Add(key, value);
                }
                for (int i = 0; i < width; i++)
                {
                    value.val[i] = int.Parse($"{value.val[i]}{line[currPos + i]}");
                }

                currPos += width + 1;
            }
        }
        return CalcSolution(columns, operations);
    }

    static Dictionary<int, (string op, int pos)> ParseOperations(string line)
    {
        Dictionary<int, (string op, int pos)> operations = new Dictionary<int, (string, int)>();
        int currWidth = 0;
        int col = 0;
        foreach (char c in line)
        {
            currWidth++;
            if (c == ' ')
                continue;
            operations[col] = (c.ToString(), currWidth);
            col++;
        }

        foreach (int key in operations.Keys)
        {
            var (symbol, width) = operations[key];
            int newWidth = line.Length - width + 1;
            if (operations.ContainsKey(key + 1)) newWidth = operations[key + 1].pos - 1 - width;

            operations[key] = (symbol, newWidth);
        }
        return operations;
    }

    static long CalcSolution(Dictionary<int, (string symbol, int[] val)> columns, Dictionary<int, (string op, int pos)> operations)
    {
        long solution = 0;
        foreach (int key in columns.Keys)
        {
            var (symbol, _) = operations[key];
            long total = symbol == "*" ? 1 : 0;

            foreach (var val in columns[key].val)
            {
                switch (symbol)
                {
                    case "+":
                        total += val;
                        break;
                    case "*":
                        total *= val;
                        break;
                }
            }
            solution += total;
        }
        return solution;
    }
}