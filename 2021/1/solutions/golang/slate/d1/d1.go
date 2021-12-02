package d1

import (
	"log"
	"strconv"
)

type D1 struct {
}

func (d D1) PartOne(in []string) int {

	var counter int
	text := in

	for i := range text {
		if i == 0 {
			continue
		}
		current, err := strconv.Atoi(text[i])
		if err != nil {
			log.Println(err)
		}
		past, err := strconv.Atoi(text[i-1])
		if err != nil {
			log.Println(err)
		}
		if current > past {
			counter += 1
		}
	}
	return counter
}

func (d D1) PartTwo(input []string) int {
	var counter int

	text := input

	for i := range text {
		if i == 0 || i == 1 || i == 2 {
			continue
		}
		ctotal := getText(text, i) + getText(text, i-1) + getText(text, i-2)
		cpast2 := getText(text, i-1) + getText(text, i-2) + getText(text, i-3)
		if ctotal > cpast2 {
			counter += 1
		}
	}
	return counter
}

func getText(text []string, i int) int {
	number, err := strconv.Atoi(text[i])
	if err != nil {
		log.Println(err)
	}
	return number
}
