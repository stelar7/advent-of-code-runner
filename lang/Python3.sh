#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

SOLUTION=$1
IO_FILES=$2

START=$($D/util/start.sh)

while read INPUT OUTPUT; do
    CURRENT=$($D/util/start.sh)

    cat $INPUT | timeout --signal=SIGKILL 20s python3 $SOLUTION | diff --strip-trailing-cr $OUTPUT - >/dev/null
    if [ $? -ne 0 ]; then
        $D/util/error.sh "Python3" "$SOLUTION" "$INPUT" "$($D/util/stop.sh $CURRENT)" "0"
        exit
    fi

    TIMES+=($($D/util/stop.sh $CURRENT))
done < <(echo $IO_FILES | xargs -n2)

TOTAL=$($D/util/stop.sh $START)

$D/util/success.sh "Python3" "$TOTAL" "$SOLUTION" "$TIMES" "0"
