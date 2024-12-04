# Djot Implementations

This repository benchmarks the various Djot implementations and generates a report.

## Benchmarks

Time for parsing pandoc-manual.dj (244K) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 6.0 ± 0.1 | 5.8 | 8.2 | 1.00 |
| `Haskell` | 34.8 ± 1.1 | 33.6 | 38.0 | 5.76 ± 0.21 |
| `Go` | 35.4 ± 2.0 | 30.8 | 41.5 | 5.86 ± 0.35 |
| `JavaScript` | 127.7 ± 4.1 | 117.6 | 138.6 | 21.13 ± 0.78 |
| `Lua` | 149.7 ± 1.9 | 146.3 | 152.6 | 24.77 ± 0.57 |

Time for parsing tartan-wikipedia.dj (1.3M) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 34.6 ± 0.4 | 34.0 | 36.3 | 1.00 |
| `Go` | 214.0 ± 9.1 | 198.8 | 229.2 | 6.19 ± 0.27 |
| `Haskell` | 235.5 ± 1.6 | 232.4 | 238.5 | 6.81 ± 0.09 |
| `JavaScript` | 394.5 ± 4.8 | 386.5 | 404.0 | 11.41 ± 0.19 |
| `Lua` | 1139.3 ± 6.5 | 1132.9 | 1151.9 | 32.95 ± 0.40 |

Running on AMD EPYC 7763 64-Core Processor at 2024-12-04 01:15:26+00:00  
go version go1.23.3 linux/amd64  
Go djot godjot@v1.0.5  
The Glorious Glasgow Haskell Compilation System, version 9.10.1  
Haskell djot UNKNOWN  
node version v22.11.0  
JavaScript djot 0.3.1  
Lua 5.4.4  Copyright (C) 1994-2022 Lua.org, PUC-Rio  
Lua djot 0.2.1  
rustc 1.83.0 (90b35a623 2024-11-26)  
Rust djot
jotdown v0.6.0
