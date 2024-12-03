#!/bin/sh

PANDOC_MANUAL=https://github.com/jgm/djot/files/11900633/pandoc-manual.txt
PANDOC_MANUAL_DJ=pandoc-manual.dj

wget --no-clobber --output-document=$PANDOC_MANUAL_DJ $PANDOC_MANUAL

hyperfine --warmup 20 --shell=none --export-markdown implementations.md --sort mean-time \
--command-name Go "$HOME/go/bin/godjot $PANDOC_MANUAL_DJ" \
--command-name Haskell "$HOME/.cabal/bin/djoths $PANDOC_MANUAL_DJ" \
--command-name JavaScript "djot $PANDOC_MANUAL_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $PANDOC_MANUAL_DJ" \
--command-name Rust "jotdown $PANDOC_MANUAL_DJ"
