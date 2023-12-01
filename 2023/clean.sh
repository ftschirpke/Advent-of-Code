#!/usr/bin/env bash

find . -type f -name "*.Zone.Identifier" -exec rm -vf {} \;
find . -type d -name "target" -exec rm -rd {} \;
