#!/usr/bin/env bash
set -euo pipefail

CMD="$1"
FILE="$2"
INPUT="$3"

printf "%-10s %-40s ❌ %s\n" "$CMD" "$(basename $(dirname -- $FILE))" "$INPUT"