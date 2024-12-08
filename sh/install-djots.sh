#!/bin/dash

# Go
go -v -x install github.com/sivukhin/godjot@latest # Installed in: ~/go/bin/godjot

# Haskell
cabal update
cabal install djot  --upgrade-dependencies --overwrite-policy=always # Installed in: ~/.cabal/bin/djoths

# JavaScript
npm install --omit=dev -g @djot/djot@latest # Installed in: ~/.nvm/versions/node/v22.11.0/bin/djot

# Lua
luarocks install --no-doc --local djot # Installed in: ~/.luarocks/bin/djot

# Rust
rustup update stable
rustup default stable
cargo install jotdown # Installed in: ~/.cargo/bin/jotdown

# Hyperfine
cargo install hyperfine # Installed in: ~/.cargo/bin/hyperfine