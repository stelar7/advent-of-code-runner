#!/usr/bin/env bash
set -euo pipefail

D=$(dirname $(realpath $0))

echo "-- Day 5 --"
printf "%-10s %-15s %-10s %-10s %-10s %-10s \n" "language" "author" "compile" "total" "average" "status"
$D/../../lang/node.sh                    "$D/solutions/node/stelar7/main.mjs"              "$D/io/*"
$D/../../lang/cargo_rust.sh              "$D/solutions/rust/thedrone7"                     "$D/io/*"
$D/../../lang/cargo_rust.sh              "$D/solutions/rust/FX"                            "$D/io/*"
$D/../../lang/cmake_cpp.sh               "$D/solutions/cpp/pseudonym117"                   "$D/io/*"
$D/../../lang/python3.sh                 "$D/solutions/python/fsantos98/day5.py"           "$D/io/*"

echo ""
