#!/bin/bash

# sissa.it has some pretty strict DOS prevention in place.
# This script simulates a fresh download of the data.

cd $(git rev-parse --show-toplevel)

dir="/home/simon/.local/share/parsec_access_0.1.0"
rm -rf $dir
mkdir -p $dir
find dev_data -type d -name "Z*" -exec cp -r {} $dir \;
ls -l $dir
