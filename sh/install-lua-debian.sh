#!/bin/sh -e

# To install Lua:
sudo apt-get --yes install lua5.4 liblua5.4-dev

# To install LuaRocks:
wget --quiet --no-clobber https://luarocks.org/releases/luarocks-3.11.1.tar.gz || true
tar zxpf luarocks-3.11.1.tar.gz
cd luarocks-3.11.1
./configure && make && sudo make install
cd ../
