#!/bin/bash

set -v

./build.sh

dest="../ivanceras.github.io/hackernews"

rm -rf "$dest"

mkdir -p "$dest"

cp -r client/index.html client/style.css client/pkg "$dest/"

## Remove the ignore file on the pkg directory
rm $dest/pkg/.gitignore
