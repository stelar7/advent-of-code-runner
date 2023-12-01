for X in {1..25..1} 
do 
mkdir "2023/$X" 
mkdir "2023/$X/io" 
mkdir "2023/$X/solutions" 
touch "2023/$X/README.md"
cp lang/util/base_test_script.sh "2023/$X/test.sh"
sed -i "s/Day 1/Day $X/g" "2023/$X/test.sh"
chmod +x "2023/$X/test.sh"

done