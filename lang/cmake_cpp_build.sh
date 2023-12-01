#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1
LOGPATH="$D/../logs/$(basename $(dirname $SOLUTION))/${SOLUTION##*/}"
mkdir -p $LOGPATH

rm -rf $SOLUTION/build
mkdir $SOLUTION/build
cd $SOLUTION/build

START=$($D/util/start.sh)

timeout --signal=SIGKILL 120s cmake -G Ninja -DCMAKE_CXX_COMPILER_LAUNCHER=ccache -DCMAKE_BUILD_TYPE=Release .. > $LOGPATH/cmake.log 2>&1
timeout --signal=SIGKILL 120s ninja > $LOGPATH/ninja.log 2>&1

echo $($D/util/stop.sh $START)