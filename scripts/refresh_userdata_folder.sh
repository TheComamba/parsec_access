#!/bin/bash

# sissa.it has some pretty strict DOS prevention in place.
# This script simulates a fresh download of the data.

cd $(git rev-parse --show-toplevel)

package_version=$(cargo read-manifest | jq -r .version)

dir="/home/simon/.local/share/parsec_access_$package_version"
rm -rf $dir
mkdir -p $dir
find dev_data -type d -name "Z*" -exec cp -r {} $dir \;
ls -l $dir
