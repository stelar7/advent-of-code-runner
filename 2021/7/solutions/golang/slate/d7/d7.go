package d7

import (
	"log"
	"math"
	"sort"
	"strconv"
	"strings"
)

type D7 struct {
}

func (d D7) PartOne(in []string) (int, int) {
	input := strings.Split(in[0], ",")

	var median []int

	for _, i := range input {
		medianint, err := strconv.Atoi(i)
		if err != nil {
			log.Fatal(err)
		}
		median = append(median, medianint)
	}

	sort.Ints(median)
	mid := int(len(median) / 2)

	fuel := 0

	gold := median[mid]

	for _, i := range median {
		if i > gold {
			dist := i - gold
			fuel += dist
		} else if gold > i {
			dist := gold - i
			fuel += dist
		}
	}

	gold = 0
	gold_counter := 0

	for _, med := range median {
		gold += med
		gold_counter += 1
	}

	gold = int(math.Round(float64(((gold * 999) / gold_counter)) / 1000))

	fuel2 := 0
	for _, i := range median {
		if i > gold {
			dist := i - gold
			for j := 0; j <= dist; j++ {
				fuel2 += j
			}
		} else if gold > i {
			dist := gold - i
			for j := 0; j <= dist; j++ {
				fuel2 += j
			}
		}
	}
	return fuel, fuel2
}
