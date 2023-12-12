#!/bin/bash
set -xeuo pipefail

rm -rf $NEW_DAY
cp -r day00 $NEW_DAY
find $NEW_DAY -type f -exec sed -i "s/day00/$NEW_DAY/g" {} \;
mv $NEW_DAY/src/bin/day00-part1.rs $NEW_DAY/src/bin/$NEW_DAY-part1.rs
mv $NEW_DAY/src/bin/day00-part2.rs $NEW_DAY/src/bin/$NEW_DAY-part2.rs
head -n-1 Cargo.toml > Cargo.toml.tmp
echo -e "    \"$NEW_DAY\",\n]" >> Cargo.toml.tmp
mv Cargo.toml.tmp Cargo.toml
