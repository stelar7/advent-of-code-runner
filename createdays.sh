YEAR=2024
mkdir "$YEAR" 
for X in $(seq 1 25);
do 
mkdir "$YEAR/$X" 
mkdir "$YEAR/$X/io" 
mkdir "$YEAR/$X/solutions" 
touch "$YEAR/$X/README.md"
cp lang/util/base_test_script.sh "$YEAR/$X/test.sh"
sed -i "s/Day 1/Day $X/g" "$YEAR/$X/test.sh"
chmod +x "$YEAR/$X/test.sh"

done