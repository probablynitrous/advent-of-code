using AoC.Solutions;
namespace AoC;
class Program
{
    static async Task Main(string[] args)
    {
        Console.WriteLine("Select day to run:");
        string? dayInput = Console.ReadLine();
        Console.WriteLine("Run test data? (y/n)");
        bool isTest = Console.ReadLine()?.ToLower() == "y";
        if (string.IsNullOrWhiteSpace(dayInput))
        {
            Console.WriteLine("No day provided.");
            return;
        }

        string day = dayInput;
        string result = "";
        switch (day)
        {
            case "1":
                result = await DayOne.Run(isTest);
                break;
            case "2":
                result = await DayTwo.Run(isTest);
                break;
            case "3":
                result = await DayThree.Run(isTest);
                break;
            case "4":
                result = await DayFour.Run(isTest);
                break;
            case "5":
                result = await DayFive.Run(isTest);
                break;
            case "6":
                result = await DaySix.Run(isTest);
                break;
        }

        Console.WriteLine(result);
        Console.Read();
    }
}