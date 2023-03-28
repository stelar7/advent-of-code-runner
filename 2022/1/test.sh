#!/usr/bin/env bash
set -euo pipefail

D=$(dirname $(realpath $0))

echo "-- Day 1 --"
printf "%-10s %-15s %-10s %-10s %-10s %-10s \n" "language" "author" "compile" "total" "average" "status"
$D/../../lang/osabie.sh                   "$D/solutions/05ab1e/stelar7/day1.abe"            "$D/io/*"
$D/../../lang/golfscript.sh               "$D/solutions/golfscript/pseudonym117/day1.gs"    "$D/io/*"
$D/../../lang/python3.sh		  "$D/solutions/python/slate/d1.py"		    "$D/io/*"
$D/../../lang/cargo_rust.sh              "$D/solutions/rust/mingweisamuel"                  "$D/io/*"

echo ""
