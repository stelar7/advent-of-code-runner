#!/usr/bin/env bash
set -euo pipefail

D=$(dirname $(realpath $0))

echo "-- Day 0 --"
printf "%-10s %-15s %-10s %-10s %-10s %-10s \n" "language" "author" "compile" "total" "average" "status"
$D/../../lang/node.sh       "$D/solutions/node/example/main.mjs"        "$D/io/*"
$D/../../lang/node.sh       "$D/solutions/node/slow/main.mjs"           "$D/io/*"
$D/../../lang/node.sh       "$D/solutions/node/wrong/main.mjs"          "$D/io/*"
$D/../../lang/node.sh       "$D/solutions/node/too-slow/main.mjs"       "$D/io/*"

echo ""
