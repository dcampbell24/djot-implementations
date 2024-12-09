#!/bin/dash -e

mkdir tmp || true

PANDOC_MANUAL=https://github.com/jgm/djot/files/11900633/pandoc-manual.txt
PANDOC_MANUAL_DJ=tmp/pandoc-manual.dj

wget --quiet --no-clobber --output-document=$PANDOC_MANUAL_DJ $PANDOC_MANUAL || true

[ -s "$HOME/.nvm/nvm.sh" ] && \. "$HOME/.nvm/nvm.sh"
DJOT_JS=$(echo $(nvm which current) | sed -e 's/node$/djot/')

hyperfine --warmup 20 --shell=none --export-markdown tmp/pandoc-manual-benchmarks.md \
--sort mean-time \
--export-json tmp/pandoc-manual-benchmarks.json \
--command-name Go "$HOME/go/bin/godjot -from $PANDOC_MANUAL_DJ" \
--command-name Haskell "djoths $PANDOC_MANUAL_DJ" \
--command-name JavaScript "$DJOT_JS $PANDOC_MANUAL_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $PANDOC_MANUAL_DJ" \
--command-name Rust "jotdown $PANDOC_MANUAL_DJ"

TARTAN_WIKIPEDIA=https://en.wikipedia.org/wiki/Tartan
TARTAN_WIKIPEDIA_HTML=tmp/tartan-wikipedia.html
TARTAN_WIKIPEDIA_DJ=tmp/tartan-wikipedia.dj

wget --quiet --no-clobber --output-document=$TARTAN_WIKIPEDIA_HTML $TARTAN_WIKIPEDIA || true
pandoc --from=html --to=djot --output=$TARTAN_WIKIPEDIA_DJ $TARTAN_WIKIPEDIA_HTML

hyperfine --warmup 20 --shell=none --export-markdown tmp/tartan-wikipedia-benchmarks.md \
--sort mean-time \
--export-json tmp/tartan-wikipedia-benchmarks.json \
--command-name Go "$HOME/go/bin/godjot -from $TARTAN_WIKIPEDIA_DJ" \
--command-name Haskell "djoths $TARTAN_WIKIPEDIA_DJ" \
--command-name JavaScript "$DJOT_JS $TARTAN_WIKIPEDIA_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $TARTAN_WIKIPEDIA_DJ" \
--command-name Rust "jotdown $TARTAN_WIKIPEDIA_DJ"

cat > tmp/benchmarks.md <<EOF
Time to render pandoc-manual.dj ($(echo $(du -h tmp/pandoc-manual.dj) | sed -r 's/([0-9.MK]+)[ .a-zA-Z/-]+/\1/')) into html:

$(cat tmp/pandoc-manual-benchmarks.md)

Time to render tartan-wikipedia.dj ($(echo $(du -h tmp/tartan-wikipedia.dj) | sed -r 's/([0-9.MK]+)[ .a-zA-Z/-]+/\1/')) into html:

$(cat tmp/tartan-wikipedia-benchmarks.md)

Running on $(lscpu | grep "Model name: [-|(|)| |a-z|A-Z|0-9]*" | sed -e 's/Model name: *//') at $(date --rfc-3339=seconds)  
$(go version)  
Go djot $(ls $HOME/go/pkg/mod/github.com/sivukhin/)  
$(ghc --version)  
Haskell djot $(ls $HOME/.cabal/packages/hackage.haskell.org/djot/)  
node version $(node --version)  
JavaScript $(djot --version)  
$(lua -v)  
Lua $($HOME/.luarocks/bin/djot --version)  
$(rustc --version)
EOF

jotdown --version 2>> tmp/benchmarks.md
