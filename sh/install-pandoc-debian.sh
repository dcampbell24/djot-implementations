#!/bin/sh -e

wget --quiet https://github.com/jgm/pandoc/releases/download/3.5/pandoc-3.5-1-amd64.deb
sudo dpkg --install pandoc-3.5-1-amd64.deb
