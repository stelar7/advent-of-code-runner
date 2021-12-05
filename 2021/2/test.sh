#!/usr/bin/env bash
set -euo pipefail

D=$(dirname $(realpath $0))

echo "-- Day 2 --"
printf "%-10s %-15s %-10s %-10s %-10s %-10s \n" "language" "author" "compile" "total" "average" "status"
$D/../../lang/node.sh				    "$D/solutions/node/stelar7/main.mjs"	    "$D/io/*"
$D/../../lang/cmake_cpp.sh			    "$D/solutions/cmake_cpp/querijn/"	        "$D/io/*"
$D/../../lang/python3.sh        	    "$D/solutions/python/kurainu/main.py"       "$D/io/*"
$D/../../lang/cargo_rust.sh			    "$D/solutions/rust/molenzwiebel"	        "$D/io/*"
$D/../../lang/cargo_rust.sh			    "$D/solutions/rust/mingweisamuel"	        "$D/io/*"
$D/../../lang/dotnet_c_sharp.sh		    "$D/solutions/dotnet_c_sharp/aoshiw"	    "$D/io/*"
$D/../../lang/golang.sh				    "$D/solutions/golang/slate/"		        "$D/io/*"
$D/../../lang/perl.sh				    "$D/solutions/perl/slate/day2.pl"	    "$D/io/*"
$D/../../lang/dotnet_c_sharp.sh		    "$D/solutions/dotnet_c_sharp/renkon"	    "$D/io/*"
echo ""
