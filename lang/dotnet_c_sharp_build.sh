#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1

OUTPUT="$(dirname $SOLUTION)/build"
rm -rf $OUTPUT

START=$($D/util/start.sh)

timeout --signal=SIGKILL 120s dotnet build -o "$OUTPUT" "$SOLUTION"

echo $($D/util/stop.sh $START)