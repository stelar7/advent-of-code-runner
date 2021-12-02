#!/usr/bin/env bash
set -euo pipefail

D=$(dirname $(realpath $0))


$D/../../lang/cmake_cpp_build.sh	"$D/solutions/cmake_cpp/querijn"

echo "-- Day 1 --"
printf "%-10s %-20s %-8s %-10s %-10s \n" "language" "author" "total" "average" "status"
$D/../../lang/cmake_cpp.sh			"$D/solutions/cmake_cpp/querijn"		"$D/io/*"

echo ""
