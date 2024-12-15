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

### Running on AMD EPYC 7763 64-Core Processor at 2024-12-15 03:15:54+00:00

Time to render pandoc-manual.dj (244K) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 6.1 ± 0.1 | 5.9 | 6.4 | 1.00 |
| `Go` | 36.1 ± 2.7 | 32.0 | 42.7 | 5.88 ± 0.44 |
| `Haskell` | 42.3 ± 1.7 | 32.2 | 52.5 | 6.89 ± 0.29 |
| `Lua` | 107.9 ± 3.5 | 102.5 | 113.8 | 17.59 ± 0.61 |
| `JavaScript` | 118.0 ± 3.8 | 111.6 | 124.0 | 19.24 ± 0.65 |

Time to render tartan-wikipedia.dj (1.3M) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 34.4 ± 0.3 | 33.9 | 36.0 | 1.00 |
| `Go` | 214.9 ± 17.5 | 193.6 | 253.0 | 6.24 ± 0.51 |
| `Haskell` | 235.7 ± 0.1 | 235.6 | 236.0 | 6.85 ± 0.06 |
| `JavaScript` | 355.0 ± 5.7 | 346.5 | 365.3 | 10.32 ± 0.19 |
| `Lua` | 659.6 ± 12.4 | 645.2 | 684.5 | 19.17 ± 0.40 |

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
