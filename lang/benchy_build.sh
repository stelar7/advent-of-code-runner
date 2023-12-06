#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

cd $D/util/bench

timeout --signal=SIGKILL 120s cmake -G Ninja -DCMAKE_CXX_COMPILER_LAUNCHER=ccache -DCMAKE_BUILD_TYPE=Release .
timeout --signal=SIGKILL 120s ninja