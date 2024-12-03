#!/bin/sh

PANDOC_MANUAL=https://github.com/jgm/djot/files/11900633/pandoc-manual.txt
PANDOC_MANUAL_DJ=pandoc-manual.dj

wget --no-clobber --output-document=$PANDOC_MANUAL_DJ $PANDOC_MANUAL

hyperfine --warmup 20 --shell=none --sort mean-time \
--command-name Go "$HOME/go/bin/godjot $PANDOC_MANUAL_DJ" \
--command-name Haskell "$HOME/.cabal/bin/djoths $PANDOC_MANUAL_DJ" \
--command-name JavaScript "djot $PANDOC_MANUAL_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $PANDOC_MANUAL_DJ" \
--command-name Rust "jotdown $PANDOC_MANUAL_DJ"

TARTAN_WIKIPEDIA=https://en.wikipedia.org/wiki/Tartan
TARTAN_WIKIPEDIA_HTML=tartan-wikipedia.html
TARTAN_WIKIPEDIA_DJ=tartan-wikipedia.dj

wget --no-clobber --output-document=$TARTAN_WIKIPEDIA_HTML $TARTAN_WIKIPEDIA
pandoc --from=html --to=djot --output=$TARTAN_WIKIPEDIA_DJ $TARTAN_WIKIPEDIA_HTML

hyperfine --warmup 20 --shell=none --sort mean-time \
--command-name Go "$HOME/go/bin/godjot $TARTAN_WIKIPEDIA_DJ" \
--command-name Haskell "$HOME/.cabal/bin/djoths $TARTAN_WIKIPEDIA_DJ" \
--command-name JavaScript "djot $TARTAN_WIKIPEDIA_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $TARTAN_WIKIPEDIA_DJ" \
--command-name Rust "jotdown $TARTAN_WIKIPEDIA_DJ"