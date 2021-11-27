#!/usr/bin/env bash
set -uo pipefail

INPUT="$1"

COUNT=0
SUM=0
for i in "${INPUT[@]}"
do
    ((SUM += $i))
    ((COUNT++))
done

echo $((SUM/COUNT))