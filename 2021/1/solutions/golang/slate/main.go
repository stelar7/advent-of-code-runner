package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/5late/aoc/d1"
)

func main() {
	var DayOne d1.D1

	reader := bufio.NewScanner(os.Stdin)
	input := make([]string, 0)

	for reader.Scan() {
		input = append(input, reader.Text())
	}

	fmt.Print(DayOne.PartOne(input), "\n")
	fmt.Print(DayOne.PartTwo(input), "\n")
}
