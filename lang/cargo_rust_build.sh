#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1
LOGPATH="$D/../logs/$(basename $(dirname $SOLUTION))/${SOLUTION##*/}"
mkdir -p $LOGPATH

rm -rf $SOLUTION/target

cargo install lazy_static > $LOGPATH/cargo_install.log 2>&1

START=$($D/util/start.sh)

timeout --signal=SIGKILL 120s cargo build --manifest-path "$SOLUTION/Cargo.toml" --release > $LOGPATH/cargo_build.log 2>&1

echo $($D/util/stop.sh $START)