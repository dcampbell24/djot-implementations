#!/bin/bash -e

# To install Lua:
sudo apt-get --yes install lua5.4 liblua5.4-dev

# To install LuaRocks:
wget https://luarocks.org/releases/luarocks-3.13.0.tar.gz
tar zxpf luarocks-3.13.0.tar.gz
cd luarocks-3.13.0
./configure && make && sudo make install
cd ../
