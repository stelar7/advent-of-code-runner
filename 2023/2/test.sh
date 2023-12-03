#!/usr/bin/env bash
set -euo pipefail

D=$(dirname $(realpath $0))

echo "-- Day 2 --"
printf "%-10s %-15s %-10s %-10s %-10s %-10s \n" "language" "author" "compile" "total" "average" "status"
$D/../../lang/cmake_cpp.sh               "$D/solutions/cpp/querijn"                        "$D/io/*"
$D/../../lang/elixir.sh                  "$D/solutions/elixir/leastrio/main.exs"           "$D/io/*"
$D/../../lang/node.sh                    "$D/solutions/node/stelar7/main.mjs"              "$D/io/*"
$D/../../lang/cargo_rust.sh              "$D/solutions/rust/thedrone7"		                 "$D/io/*"
$D/../../lang/cargo_rust.sh              "$D/solutions/rust/FX"                            "$D/io/*"

echo ""
