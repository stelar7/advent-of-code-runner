package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/5late/aoc/d6"
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

	/*var DayTwo d2.D2

	reader := bufio.NewScanner(os.Stdin)
	input := make([]string, 0)

	for reader.Scan() {
		input = append(input, reader.Text())
	}

	horiz, depth, total := DayTwo.PartOne(input)
	horizon, deep, totaldif := DayTwo.PartTwo(input)
	fmt.Print(horiz, depth, total, "\n")
	fmt.Print(horizon, deep, totaldif, "\n")

	var DayThree d3.D3

	reader := bufio.NewScanner(os.Stdin)
	input := make([]string, 0)

	for reader.Scan() {
		input = append(input, reader.Text())
	}

	answer := DayThree.PartOne(input)
	oxdecimal, err := strconv.ParseInt(fmt.Sprint(DayThree.PartTwo(0, input, "ox")), 2, 64)
	if err != nil {
		log.Fatal(err)
	}
	codecimal, err := strconv.ParseInt(fmt.Sprint(DayThree.PartTwo(0, input, "co")), 2, 64)
	if err != nil {
		log.Fatal(err)
	}
	//horizon, deep, totaldif := D.PartTwo(input)
	fmt.Println(answer)
	fmt.Println(oxdecimal * codecimal)*/
	/*
		var DayFour d4.D4

		reader := bufio.NewScanner(os.Stdin)
		input := make([]string, 0)

		for reader.Scan() {
			input = append(input, reader.Text())
		}

		answer := DayFour.PartOne(input)
		//horizon, deep, totaldif := D.PartTwo(input)
		fmt.Println(answer[0])*/

	var DaySix d6.D6
	reader := bufio.NewScanner(os.Stdin)
	input := make([]string, 0)

	for reader.Scan() {
		input = append(input, reader.Text())
	}

	counter, counter2 := DaySix.PartOne(input)
	fmt.Println(counter)
	fmt.Println(counter2)
}
