#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1

rm -rf $SOLUTION/target

cargo install lazy_static > /dev/null 2>&1

START=$($D/util/start.sh)

timeout --signal=SIGKILL 120s cargo build --manifest-path "$SOLUTION/Cargo.toml" --release > /dev/null 2>&1

echo $($D/util/stop.sh $START)