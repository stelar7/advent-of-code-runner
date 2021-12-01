#!/usr/bin/env bash
set -euo pipefail
D=$(dirname $(realpath $0))

CMD="$1"
FILE="$2"
INPUT="$3"
TIMEOUT="$($D/timeformat.sh $4)"
COMPILETIME="$($D/timeformat.sh $5)"

printf "%-10s %-15s %-10s %-21s ❌ %s\n" "$CMD" "$(basename $(dirname -- $FILE))" "$COMPILETIME" "$TIMEOUT" "$INPUT"