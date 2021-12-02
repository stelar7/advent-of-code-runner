#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1

rm -rf $SOLUTION/build
mkdir -p $SOLUTION/build
cd $SOLUTION/build

START=$($D/util/start.sh)

timeout --signal=SIGKILL 20s go build $SOLUTION >/dev/null

echo $($D/util/stop.sh $START)