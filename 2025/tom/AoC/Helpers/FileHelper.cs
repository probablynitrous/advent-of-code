using System.Reflection;

namespace AoC.Helpers;

public class FileHelper
{
    public static async Task<string[]> ReadFile(string fileName)
    {
        var assembly = Assembly.GetExecutingAssembly();

        using (Stream? stream = assembly.GetManifestResourceStream(fileName))
        {
            if (stream == null)
            {
                throw new FileNotFoundException($"Embedded resource not found: {fileName}");
            }

            using (StreamReader reader = new StreamReader(stream))
            {
                return reader.ReadToEnd().Split(new[] { "\r\n", "\r", "\n" }, StringSplitOptions.None);
            }
        }
    }
}