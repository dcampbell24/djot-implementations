#!/bin/bash

mkdir djot html markdown

PANDOC_MANUAL=https://github.com/jgm/djot/files/11900633/pandoc-manual.txt
PANDOC_MANUAL_DJ=djot/pandoc-manual.dj

wget --quiet --no-clobber --output-document=$PANDOC_MANUAL_DJ $PANDOC_MANUAL

hyperfine --warmup 20 --shell=none --export-markdown markdown/benchmarks-1.md --sort mean-time \
--command-name Go "$HOME/go/bin/godjot -from $PANDOC_MANUAL_DJ" \
--command-name Haskell "djoths $PANDOC_MANUAL_DJ" \
--command-name JavaScript "djot $PANDOC_MANUAL_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $PANDOC_MANUAL_DJ" \
--command-name Rust "jotdown $PANDOC_MANUAL_DJ"

TARTAN_WIKIPEDIA=https://en.wikipedia.org/wiki/Tartan
TARTAN_WIKIPEDIA_HTML=html/tartan-wikipedia.html
TARTAN_WIKIPEDIA_DJ=djot/tartan-wikipedia.dj

wget --quiet --no-clobber --output-document=$TARTAN_WIKIPEDIA_HTML $TARTAN_WIKIPEDIA
pandoc --from=html --to=djot --output=$TARTAN_WIKIPEDIA_DJ $TARTAN_WIKIPEDIA_HTML

hyperfine --warmup 20 --shell=none --export-markdown markdown/benchmarks-2.md --sort mean-time \
--command-name Go "$HOME/go/bin/godjot -from $TARTAN_WIKIPEDIA_DJ" \
--command-name Haskell "djoths $TARTAN_WIKIPEDIA_DJ" \
--command-name JavaScript "djot $TARTAN_WIKIPEDIA_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $TARTAN_WIKIPEDIA_DJ" \
--command-name Rust "jotdown $TARTAN_WIKIPEDIA_DJ"

cat > markdown/benchmarks.md <<EOF
Time for parsing pandoc-manual.dj ($(echo $(du -h djot/pandoc-manual.dj) | sed -r 's/([0-9.MK]+)[ .a-zA-Z/-]+/\1/')) into html:

$(cat markdown/benchmarks-1.md)

Time for parsing tartan-wikipedia.dj ($(echo $(du -h djot/tartan-wikipedia.dj) | sed -r 's/([0-9.MK]+)[ .a-zA-Z/-]+/\1/')) into html:

$(cat markdown/benchmarks-2.md)

Running on $(lscpu | grep "Model name: [-|(|)| |a-z|A-Z|0-9]*" | sed -e 's/Model name: *//') at $(date --rfc-3339=seconds)  
$(go version)  
Go djot $(ls $HOME/go/pkg/mod/github.com/sivukhin/)  
$(ghc --version)  
Haskell djot UNKNOWN  
node version $(node --version)  
JavaScript $(djot --version)  
$(lua -v)  
Lua $($HOME/.luarocks/bin/djot --version)  
$(rustc --version)
EOF

jotdown --version 2>> markdown/benchmarks.md
