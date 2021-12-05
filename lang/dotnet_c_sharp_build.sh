#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1

rm -rf "$SOLUTION/obj"
rm -rf "$SOLUTION/bin"

START=$($D/util/start.sh)

timeout --signal=SIGKILL 120s dotnet build --configuration Release "$SOLUTION" >/dev/null

echo $($D/util/stop.sh $START)