package d2

import (
	"log"
	"strconv"
)

type D2 struct {
}

func (d D2) PartOne(in []string) (int, int, int) {
	var horiz int
	var depth int

	text := in

	for i := range text {
		if len(text[i]) == 9 {
			horizon, err := strconv.Atoi(text[i][len(text[i])-1:])
			if err != nil {
				log.Fatal(err)
			}
			horiz += horizon
		} else if len(text[i]) == 6 {
			deep, err := strconv.Atoi(text[i][len(text[i])-1:])
			if err != nil {
				log.Fatal(err)
			}
			depth += deep
		} else if len(text[i]) == 4 {
			deep, err := strconv.Atoi(text[i][len(text[i])-1:])
			if err != nil {
				log.Fatal(err)
			}
			depth -= deep
		}
	}
	return horiz, depth, (horiz * depth)
}

func (d D2) PartTwo(in []string) (int, int, int) {
	var horiz int
	var depth int
	var aim int

	text := in

	for i := range text {
		if len(text[i]) == 9 {
			// forward
			horizon, err := strconv.Atoi(text[i][len(text[i])-1:])
			if err != nil {
				log.Fatal(err)
			}
			horiz += horizon
			depth += aim * horizon
		} else if len(text[i]) == 6 {
			// down
			deep, err := strconv.Atoi(text[i][len(text[i])-1:])
			if err != nil {
				log.Fatal(err)
			}
			aim += deep
		} else if len(text[i]) == 4 {
			// up
			deep, err := strconv.Atoi(text[i][len(text[i])-1:])
			if err != nil {
				log.Fatal(err)
			}
			aim -= deep
		}
	}
	return horiz, depth, (horiz * depth)
}
