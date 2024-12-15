# Djot Implementations

This repository benchmarks the various Djot implementations and generates a report.

## Benchmarks

![pandoc manual benchmarks](benchmark-files/pandoc-manual-benchmarks.png "Pandoc Manual Benchmarks")

![tartan wikipedia benchmarks](benchmark-files/tartan-wikipedia-benchmarks.png "Tartan Wikipedia Benchmarks")

### Running on 12th Gen Intel(R) Core(TM) i5-12600K at 2024-12-14 21:51:05-05:00  

Time to render pandoc-manual.dj (244K) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 4.5 ± 1.4 | 3.1 | 13.6 | 1.00 |
| `Haskell` | 23.5 ± 4.7 | 20.9 | 42.0 | 5.26 ± 1.98 |
| `JavaScript` | 56.2 ± 1.4 | 54.2 | 62.9 | 12.61 ± 4.02 |
| `Lua` | 62.7 ± 1.4 | 60.0 | 66.7 | 14.05 ± 4.48 |
| `Go` | 66.4 ± 22.1 | 20.3 | 108.5 | 14.89 ± 6.85 |

Time to render tartan-wikipedia.dj (1.3M) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 19.9 ± 0.4 | 19.0 | 21.6 | 1.00 |
| `Haskell` | 132.6 ± 0.1 | 132.3 | 132.9 | 6.67 ± 0.13 |
| `JavaScript` | 171.5 ± 1.3 | 169.2 | 174.2 | 8.63 ± 0.18 |
| `Go` | 182.5 ± 30.8 | 144.6 | 269.1 | 9.18 ± 1.56 |
| `Lua` | 374.6 ± 6.5 | 365.1 | 388.0 | 18.85 ± 0.49 |

### Tools

go version go1.23.3 linux/amd64  
Go djot godjot@v1.0.5  
The Glorious Glasgow Haskell Compilation System, version 9.4.8  
Haskell djot 0.1.2.2  
node version v23.3.0  
JavaScript djot 0.3.1  
LuaJIT 2.1.1732813678  
Lua djot 0.2.1  
rustc 1.83.0 (90b35a623 2024-11-26)  
jotdown v0.7.0
