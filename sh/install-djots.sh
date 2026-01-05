#!/bin/bash -e

# Go
go install -v github.com/sivukhin/godjot@latest # Installed in: ~/go/bin/godjot

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
npm install --omit=dev --global @djot/djot@latest # Installed in: ~/.nvm/versions/node/v22.11.0/bin/djot

# Broken
# Lua
# luarocks install --no-doc --local djot # Installed in: ~/.luarocks/bin/djot

# Rust
rustup update stable
rustup default stable
cargo install jotdown # Installed in: ~/.cargo/bin/jotdown

# PHP
mkdir -p ~/.local/djot-php && cd ~/.local/djot-php
composer require php-collective/djot
# Create CLI wrapper
cat > djot-php <<'EOFPHP'
#!/usr/bin/env php
<?php
require __DIR__ . '/vendor/autoload.php';

use Djot\DjotConverter;

if ($argc < 2) {
    fwrite(STDERR, "Usage: djot-php <file.dj>\n");
    exit(1);
}

$file = $argv[1];
if (!file_exists($file)) {
    fwrite(STDERR, "File not found: $file\n");
    exit(1);
}

$content = file_get_contents($file);
$converter = new DjotConverter();
echo $converter->convert($content);
EOFPHP
chmod +x djot-php
cd - > /dev/null

# Hyperfine
cargo install hyperfine # Installed in: ~/.cargo/bin/hyperfine
