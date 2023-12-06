#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1
IO_FILES=$2

$D/util/bench/benchy "C++" "$SOLUTION" "$IO_FILES" "build/out" "$D/benchy_cpp_build.sh"