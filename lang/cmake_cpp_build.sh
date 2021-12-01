#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION_FILES=$1

for SOLUTION in $SOLUTION_FILES
do
    START=$($D/util/start.sh)

    mkdir $SOLUTION/build
    cd $SOLUTION/build
    cmake -DCMAKE_BUILD_TYPE=Release ..
    make

done