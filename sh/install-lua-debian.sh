#!/bin/sh -e

# To install Lua:
sudo apt-get --yes install luajit

# To install LuaRocks:
wget https://luarocks.org/releases/luarocks-3.11.1.tar.gz
tar zxpf luarocks-3.11.1.tar.gz
cd luarocks-3.11.1
./configure && make && sudo make install
cd ../
