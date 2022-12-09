#!/usr/bin/env bash
set -euo pipefail

D=$(dirname $(realpath $0))

echo "-- Day 2 --"
printf "%-10s %-15s %-10s %-10s %-10s %-10s \n" "language" "author" "compile" "total" "average" "status"
$D/../../lang/node.sh                   "$D/solutions/node/stelar7/main.mjs"            "$D/io/*"
$D/../../lang/golfscript.sh             "$D/solutions/golfscript/pseudonym117/day2.gs"  "$D/io/*"
$D/../../lang/python3.sh		"$D/solutions/python/slate/d2.py"		"$D/io/*"

echo ""
