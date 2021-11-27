#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

CMD="$1"
TIME="$($D/timeformat.sh $2)"
FILE="$3"
AVERAGE="$($D/timeformat.sh $4)"


printf "%-10s %-20s %-8s %-10s ✅\n" "$CMD" "$(basename $(dirname -- $FILE))" "$TIME" "$AVERAGE"