package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/5late/aoc/d2"
)

func main() {
	/*var DayOne d1.D1

	reader := bufio.NewScanner(os.Stdin)
	input := make([]string, 0)

	for reader.Scan() {
		input = append(input, reader.Text())
	}

	fmt.Print(DayOne.PartOne(input), "\n")
	fmt.Print(DayOne.PartTwo(input), "\n")*/

	var DayTwo d2.D2

	reader := bufio.NewScanner(os.Stdin)
	input := make([]string, 0)

	for reader.Scan() {
		input = append(input, reader.Text())
	}

	_ , _ , total := DayTwo.PartOne(input)
	_ , _ , totaldif := DayTwo.PartTwo(input)
	fmt.Println(total)
	fmt.Println(totaldif)
}
