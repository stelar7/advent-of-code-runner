#!/usr/bin/env bash
set -euo pipefail

D=$(dirname $(realpath $0))

echo "-- Day 0 --"
printf "%-10s %-20s %-8s %-10s %-10s \n" "language" "author" "total" "average" "status"
$D/../../lang/node.sh   "$D/solutions/node/example/*.mjs"  "$D/io/*"
$D/../../lang/node.sh   "$D/solutions/node/other/*.mjs"  "$D/io/*"
$D/../../lang/node.sh   "$D/solutions/node/slow/*.mjs"  "$D/io/*"

echo ""
