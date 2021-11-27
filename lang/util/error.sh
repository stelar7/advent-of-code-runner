#!/usr/bin/env bash
set -euo pipefail
D=$(dirname $(realpath $0))

CMD="$1"
FILE="$2"
INPUT="$3"
TIMEOUT="$($D/timeformat.sh $4)"

printf "%-10s %-20s %-19s ❌ %s\n" "$CMD" "$(basename $(dirname -- $FILE))" "$TIMEOUT" "$INPUT"