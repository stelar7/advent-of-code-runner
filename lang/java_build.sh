#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1

rm -rf $SOLUTION.class

START=$($D/util/start.sh)

timeout --signal=SIGKILL 20s javac $SOLUTION.java > /dev/null 2>&1

echo $($D/util/stop.sh $START)