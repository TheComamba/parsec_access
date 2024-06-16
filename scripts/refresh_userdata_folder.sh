#!/bin/bash

# sissa.it has some pretty strict DOS prevention in place.
# This script simulates a fresh download of the data.

cd $(git rev-parse --show-toplevel)

# Remove the old data
rm -rf ~/.local/share/parsec_access
mkdir -p ~/.local/share/parsec_access
find dev_data -type d -name "Z*" -exec cp -r {} ~/.local/share/parsec_access \;
ls -l ~/.local/share/parsec_access
