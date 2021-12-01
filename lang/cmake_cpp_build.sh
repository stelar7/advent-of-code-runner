#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1

rm -rf $SOLUTION/build
mkdir $SOLUTION/build
cd $SOLUTION/build

START=$($D/util/start.sh)

timeout --signal=SIGKILL 20s cmake -G Ninja -DCMAKE_CXX_COMPILER_LAUNCHER=ccache -DCMAKE_BUILD_TYPE=Release .. >/dev/null
timeout --signal=SIGKILL 20s ninja >/dev/null

echo $($D/util/stop.sh $START)