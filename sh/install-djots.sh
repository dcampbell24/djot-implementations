#!/bin/dash

# Go
go install -v -x github.com/sivukhin/godjot@latest # Installed in: ~/go/bin/godjot

# Haskell
ghcup upgrade
ghcup install ghc
ghcup set ghc
cabal update
cabal install djot  --upgrade-dependencies --overwrite-policy=always # Installed in: ~/.cabal/bin/djoths

# JavaScript
[ -s "$HOME/.nvm/nvm.sh" ] && \. "$HOME/.nvm/nvm.sh"
nvm install node
nvm alias default node
npm install --omit=dev -g @djot/djot@latest # Installed in: ~/.nvm/versions/node/v22.11.0/bin/djot

# Lua
luarocks install --no-doc --local djot # Installed in: ~/.luarocks/bin/djot

# Rust
rustup update stable
rustup default stable
cargo install jotdown # Installed in: ~/.cargo/bin/jotdown

# Hyperfine
cargo install hyperfine # Installed in: ~/.cargo/bin/hyperfine