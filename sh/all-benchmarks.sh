#!/bin/dash -e

sh/install-djots.sh
sh/benchmarks.sh
cargo run --release --bin update-graph-data
cargo run --release --bin graph-benchmarks

cat > tmp/all-benchmarks.md <<EOF
# Djot Implementations

This repository benchmarks the various Djot implementations and generates a report.

## Benchmarks

![pandoc manual benchmarks](pandoc-manual-benchmarks.png "Pandoc Manual Benchmarks")

![tartan wikipedia benchmarks](tartan-wikipedia-benchmarks.png "Tartan Wikipedia Benchmarks")

EOF

cat tmp/benchmarks.md >> tmp/all-benchmarks.md
