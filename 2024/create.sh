#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <name>"
    exit 1
fi

name=$1

cd $(dirname $0)

if [ -d $name ]; then
    echo "Error: $name already exists"
    exit 1
fi

cp -r template $name
sed -i "s/template/$name/" $name/build.zig $name/build.zig.zon $name/src/part1.zig $name/src/part2.zig
