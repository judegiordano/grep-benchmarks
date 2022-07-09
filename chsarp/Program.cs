using System.Diagnostics;
using System.IO;
using System.Text;

internal class Program
{
    private static List<String> Search(String query, String file)
    {
        List<String> results = new List<String>();

        using (var fileStream = File.OpenRead(file))
        using (var streamReader = new StreamReader(fileStream, Encoding.UTF8))
        {
            String? line;
            while ((line = streamReader.ReadLine()) is not null)
            {
                if (line.Contains(query))
                {
                    results.Add(line);
                }
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