#!/bin/dash -e

echo $(du -h $1) | sed -r 's/([0-9.MK]+)[ .a-zA-Z/-]+/\1/'
