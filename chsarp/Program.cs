using System.Diagnostics;
using System.IO;

internal class Program
{
    private static List<String> Search(String query, String file)
    {
        List<String> results = new List<String>();
        foreach (string line in File.ReadLines(file))
        {
            if (line.Contains(query))
            {
                results.Add(line);
            }
        }
        return results;
    }

    private static void Main(string[] args)
    {
        Stopwatch watch = Stopwatch.StartNew();
        String query = args[0];
        String file = args[1];
        List<String> results = Search(query, file);
        Console.WriteLine("matches: {0}", results.Count);
        watch.Stop();
        long end = watch.ElapsedMilliseconds;
        Console.WriteLine("operation complete in {0} ms", end);
    }
}