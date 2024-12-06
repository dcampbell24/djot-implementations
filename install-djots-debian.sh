#!/bin/bash -e

# Go
go install github.com/sivukhin/godjot@latest # Installed in: ~/go/bin/godjot

# Haskell
cabal update
cabal install djot # Installed in: ~/.cabal/bin/djoths

# JavaScript
npm install -g @djot/djot # Installed in: ~/.nvm/versions/node/v22.11.0/bin/djot

# Lua
luarocks install --local djot # Installed in: ~/.luarocks/bin/djot

# Rust
rustup update stable
rustup default stable
cargo install jotdown # Installed in: ~/.cargo/bin/jotdown

# Hyperfine
cargo install hyperfine # Installed in: ~/.cargo/bin/hyperfine