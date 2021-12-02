#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1

OUTPUT="$(dirname $SOLUTION)/out.exe"
rm -rf $OUTPUT

START=$($D/util/start.sh)

timeout --signal=SIGKILL 120s mcs -out:$OUTPUT "$SOLUTION" > /dev/null 2>&1

echo $($D/util/stop.sh $START)