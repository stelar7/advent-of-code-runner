#!/usr/bin/env bash
set -euo pipefail

T=$1
M=$((T/1000/60%60))
S=$((T/1000%60))
I=$((T%1000))
(( $M > 0 )) && printf '%dm' $M
(( $S > 0 )) && printf '%ds' $S
printf '%dms\n' $I