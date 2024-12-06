#!/bin/sh

lscpu | grep "Model name: [-|(|)| |a-z|A-Z|0-9]*" | sed -e 's/Model name: *//'
