
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;

namespace Interoptopus
{
    public delegate void Run();

    public class MeasureResult
    {
        private static long _calibrationTicks = 0;
        
        long _n;
        long _totalTicks;

        public double MicroPer1000()
        {
            var nn = (double) _n;
            var tt = (double) _totalTicks;

            return (100 * tt / nn);
        }

        public MeasureResult(long n, long totalTicks)
        {
            _n = n;
            _totalTicks = totalTicks;
        }
        
        public static void Calibrate(uint n, Run r)
        {
            var result = Measure(n, r);
            _calibrationTicks = result._totalTicks;
        }

        public static MeasureResult Measure(uint n, Run r)
        {
            
            var stopwatch = new Stopwatch();
            stopwatch.Start();
            for (var i = 0; i < n; i++)
            {
                r.Invoke();
            }
            stopwatch.Stop();

            return new MeasureResult( n, stopwatch.ElapsedTicks - _calibrationTicks);
        }

    }

    class Entry
    {
        public string Name;
        public MeasureResult Result;
    }

    public class MarkdownTableWriter
    {
        private List<Entry> Entries = new List<Entry>();

        public void Add(string name, MeasureResult result)
        {
            Entries.Add(new Entry()
            {
                Name = name,
                Result = result
            });
        }

        public void Write(string file)
        {
            var header = @"
# FFI Call Overheads

The numbers below are to help FFI design decisions by giving order-of-magnitude estimates how 
expensive certain constructs are.

## Notes

- Times were determined by running the given construct N times, taking the elapsed time in ticks, 
and computing the cost per 1k invocations. 

- The time of the called function is included. 

- However, the reference project was written so that each function is _minimal_, i.e., any similar 
function you wrote, would have to at least as expensive operations if it were to do anything sensible with 
the given type. 

- The list is ad-hoc, PRs adding more tests to `Benchmark.cs` are welcome. 


## System 

The following system was used:

```
System: i9-9900K, 32 GB RAM; Windows 10
rustc: stable (i.e., 1.53 or later)
profile: --release
.NET: v3.1 (netcoreapp3.1) 
```

## Results
 
| Construct | µs per 1k calls |
| --- | --- |
";
            
            using StreamWriter sw = File.CreateText(file);
            sw.Write(header);
            foreach (var entry in Entries)
            {
                sw.WriteLine($"| `{entry.Name}` | {(long) entry.Result.MicroPer1000()} |");
            }
        }
    }
}    
