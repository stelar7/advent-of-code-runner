#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1
IO_FILES=$2

START=$($D/util/start.sh)

while read INPUT OUTPUT; do
    CURRENT=$($D/util/start.sh)

    cat $INPUT | sed 's/\r//' | timeout --signal=SIGKILL 20s golfscript $SOLUTION | diff --strip-trailing-cr $OUTPUT - 
    if [ $? -ne 0 ]; then
        $D/util/error.sh "golfscript" "$SOLUTION" "$INPUT" "$($D/util/stop.sh $CURRENT)" "0"
        exit
    fi

    TIMES+=($($D/util/stop.sh $CURRENT))
done < <(echo $IO_FILES | xargs -n2)

TOTAL=$($D/util/stop.sh $START)

$D/util/success.sh "golfscript" "$TOTAL" "$SOLUTION" "$TIMES" "0"
