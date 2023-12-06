#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

LOGPATH="$D/../logs/benchy/"
mkdir -p $LOGPATH

cd $D/util/bench

timeout --signal=SIGKILL 120s cmake -G Ninja -DCMAKE_CXX_COMPILER_LAUNCHER=ccache -DCMAKE_BUILD_TYPE=Release . > $LOGPATH/cmake.log 2>&1
timeout --signal=SIGKILL 120s ninja > $LOGPATH/ninja.log 2>&1