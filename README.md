# Djot Implementations

This repository benchmarks the various Djot implementations and generates a report.

## Benchmarks

### Pandoc Manual

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `Go` | 1.1 ± 0.1 | 1.0 | 2.6 | 1.00 |
| `Rust` | 6.0 ± 0.1 | 5.9 | 8.0 | 5.68 ± 0.41 |
| `Haskell` | 33.4 ± 0.2 | 32.9 | 34.5 | 31.40 ± 2.16 |
| `JavaScript` | 126.2 ± 4.2 | 118.2 | 132.5 | 118.58 ± 9.04 |
| `Lua` | 148.2 ± 1.0 | 146.6 | 150.2 | 139.17 ± 9.58 |

Running on AMD EPYC 7763 64-Core Processor at 2024-12-03 17:28:00+00:00  
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
