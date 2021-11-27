#!/usr/bin/env bash
set -uo pipefail
D=$(dirname $(realpath $0))

CMD="$1"
TIME="$2ms"
FILE="$3"
AVERAGE="$($D/average.sh $4)ms"


printf "%-10s %-20s %-8s %-7s    ✅\n" "$CMD" "$(basename $(dirname -- $FILE))" "$TIME" "$AVERAGE"