#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

CMD="$1"
TIME="$($D/timeformat.sh $2)"
FILE="$3"
AVERAGE="$($D/timeformat.sh $4)"
COMPILETIME="$($D/timeformat.sh $5)"


printf "%-10s %-15s %-10s %-10s %-10s ✅\n" "$CMD" "$(basename $(dirname -- $FILE))" "$COMPILETIME" "$TIME" "$AVERAGE"