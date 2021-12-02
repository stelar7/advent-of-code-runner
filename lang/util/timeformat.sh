#!/usr/bin/env bash
set -euo pipefail

T=$1
M=$((T/1000/60%60))
S=$((T/1000%60))
I=$((T%1000))
TIME=$((($M * 60000) + ($S * 1000) + $I))
printf '%dms\n' $TIME