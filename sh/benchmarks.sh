#!/bin/sh -e

mkdir tmp || true

PANDOC_MANUAL_DJ=benchmark-files/pandoc-manual.dj

[ -s "$HOME/.nvm/nvm.sh" ] && \. "$HOME/.nvm/nvm.sh"
DJOT_JS=$(echo $(nvm which current) | sed -e 's/node$/djot/')

hyperfine --warmup 20 --shell=none --export-markdown tmp/pandoc-manual-benchmarks.md \
--sort mean-time \
--export-json tmp/pandoc-manual-benchmarks.json \
--command-name Go "$HOME/go/bin/godjot -from $PANDOC_MANUAL_DJ" \
--command-name Haskell "djoths $PANDOC_MANUAL_DJ" \
--command-name JavaScript "$DJOT_JS $PANDOC_MANUAL_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $PANDOC_MANUAL_DJ" \
--command-name PHP "$HOME/.local/djot-php/djot-php $PANDOC_MANUAL_DJ" \
--command-name Rust "jotdown $PANDOC_MANUAL_DJ" \
--command-name Prolog "scryer-prolog djota-0.3.4/djota-cli.pl -- $PANDOC_MANUAL_DJ"

TARTAN_WIKIPEDIA_DJ=benchmark-files/tartan-wikipedia.dj

hyperfine --warmup 20 --shell=none --export-markdown tmp/tartan-wikipedia-benchmarks.md \
--sort mean-time \
--export-json tmp/tartan-wikipedia-benchmarks.json \
--command-name Go "$HOME/go/bin/godjot -from $TARTAN_WIKIPEDIA_DJ" \
--command-name Haskell "djoths $TARTAN_WIKIPEDIA_DJ" \
--command-name JavaScript "$DJOT_JS $TARTAN_WIKIPEDIA_DJ" \
--command-name Lua "$HOME/.luarocks/bin/djot $TARTAN_WIKIPEDIA_DJ" \
--command-name PHP "$HOME/.local/djot-php/djot-php $TARTAN_WIKIPEDIA_DJ" \
--command-name Rust "jotdown $TARTAN_WIKIPEDIA_DJ"
# Crashes my system!
# --command-name Prolog "scryer-prolog djota-0.3.4/djota-cli.pl -- $TARTAN_WIKIPEDIA_DJ"

cat > tmp/benchmarks.md <<EOF
### Running on $(lscpu | grep "Model name: [-|(|)| |a-z|A-Z|0-9]*" | sed -e 's/Model name: *//') at $(date --rfc-3339=seconds)

Time to render pandoc-manual.dj ($(echo $(du -h benchmark-files/pandoc-manual.dj) | sed -r 's/([0-9.MK]+)[ .a-zA-Z/-]+/\1/')) into html:

$(cat tmp/pandoc-manual-benchmarks.md)

Time to render tartan-wikipedia.dj ($(echo $(du -h benchmark-files/tartan-wikipedia.dj) | sed -r 's/([0-9.MK]+)[ .a-zA-Z/-]+/\1/')) into html:

$(cat tmp/tartan-wikipedia-benchmarks.md)

### Tools

$(go version)  
Go djot $(ls $HOME/go/pkg/mod/github.com/sivukhin/)  
$(ghc --version)  
Haskell djot $(ls $HOME/.cabal/packages/hackage.haskell.org/djot/)  
node version $(node --version)  
JavaScript $(djot --version)  
$(lua -v | awk '{print $1, $2}')  
Lua $($HOME/.luarocks/bin/djot --version)  
$(php --version | head -1 | awk '{print $1, $2}')  
PHP djot $(composer show -d $HOME/.local/djot-php php-collective/djot 2>/dev/null | grep versions | awk '{print $2}')
Scryer Prolog $(scryer-prolog -v)
Prolog djota v0.3.4
$(rustc --version)  
EOF

jotdown --version 2>> tmp/benchmarks.md
