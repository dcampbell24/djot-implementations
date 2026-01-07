# Djot Implementations

This repository benchmarks the various Djot implementations and generates a report.

## Benchmarks

![pandoc manual benchmarks](benchmark-files/pandoc-manual-benchmarks.png "Pandoc Manual Benchmarks")

![tartan wikipedia benchmarks](benchmark-files/tartan-wikipedia-benchmarks.png "Tartan Wikipedia Benchmarks")

### Running on 12th Gen Intel(R) Core(TM) i5-12600K at 2026-01-07 16:29:08-05:00

Time to render pandoc-manual.dj (244K) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 9.9 ± 3.8 | 3.7 | 20.2 | 1.00 |
| `Haskell` | 30.8 ± 5.9 | 21.1 | 42.3 | 3.11 ± 1.34 |
| `Go` | 58.3 ± 19.3 | 23.5 | 86.9 | 5.89 ± 2.99 |
| `JavaScript` | 63.6 ± 3.0 | 57.7 | 70.4 | 6.43 ± 2.50 |
| `PHP` | 85.6 ± 2.6 | 82.9 | 94.9 | 8.66 ± 3.34 |
| `Lua` | 89.6 ± 3.4 | 86.2 | 101.3 | 9.07 ± 3.51 |
| `Prolog` | 2888.3 ± 23.1 | 2863.3 | 2940.7 | 292.20 ± 112.52 |

Time to render tartan-wikipedia.dj (1.3M) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 20.9 ± 2.1 | 19.4 | 33.0 | 1.00 |
| `Haskell` | 134.8 ± 4.3 | 132.3 | 142.7 | 6.45 ± 0.67 |
| `Go` | 178.7 ± 23.2 | 146.2 | 231.6 | 8.54 ± 1.40 |
| `JavaScript` | 185.0 ± 5.3 | 176.9 | 196.0 | 8.84 ± 0.92 |
| `PHP` | 490.2 ± 5.5 | 482.5 | 499.4 | 23.43 ± 2.35 |
| `Lua` | 659.0 ± 11.8 | 646.5 | 681.7 | 31.50 ± 3.18 |

### Running on AMD EPYC 7763 64-Core Processor at 2026-01-06 20:08:26+00:00

Time to render pandoc-manual.dj (244K) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 6.2 ± 0.1 | 6.0 | 6.7 | 1.00 |
| `Go` | 35.4 ± 2.4 | 31.4 | 42.4 | 5.69 ± 0.39 |
| `Haskell` | 42.1 ± 0.1 | 41.8 | 42.7 | 6.77 ± 0.07 |
| `JavaScript` | 116.1 ± 1.8 | 113.5 | 119.4 | 18.67 ± 0.33 |
| `Lua` | 150.1 ± 1.3 | 147.4 | 152.6 | 24.15 ± 0.30 |
| `PHP` | 452.2 ± 4.2 | 446.8 | 458.0 | 72.75 ± 0.95 |
| `Prolog` | 5515.9 ± 21.0 | 5493.1 | 5556.1 | 887.29 ± 8.70 |

Time to render tartan-wikipedia.dj (1.3M) into html:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Rust` | 35.1 ± 0.2 | 34.7 | 35.6 | 1.00 |
| `Go` | 207.6 ± 10.5 | 190.5 | 222.3 | 5.92 ± 0.30 |
| `Haskell` | 235.7 ± 0.6 | 235.2 | 236.8 | 6.72 ± 0.04 |
| `JavaScript` | 368.7 ± 6.8 | 358.8 | 376.8 | 10.51 ± 0.20 |
| `Lua` | 1147.4 ± 12.0 | 1131.4 | 1174.3 | 32.71 ± 0.39 |
| `PHP` | 2772.1 ± 51.3 | 2700.1 | 2852.7 | 79.03 ± 1.53 |

### Tools

go version go1.23.3 linux/amd64  
Go djot godjot@v1.0.6  
The Glorious Glasgow Haskell Compilation System, version 9.6.7  
Haskell djot 0.1.2.4  
node version v25.2.1  
JavaScript djot 0.3.2  
Lua 5.4.7  
Lua djot 0.2.1  
PHP 8.4.16  
PHP djot 0.1.11  
Scryer Prolog cargo:0.10.0  
Prolog djota v0.3.4  
rustc 1.92.0 (ded5c06cf 2025-12-08)  
jotdown v0.9.1
