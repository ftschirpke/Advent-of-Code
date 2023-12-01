#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <name>"
    exit 1
fi

name=$1

cd $(dirname $0)

cp -r template $name
sed -i "s/template/$name/" $name/Cargo.toml
sed -i "s/template/$name/" $name/src/bin/part1.rs
sed -i "s/template/$name/" $name/src/bin/part2.rs
