package d6

import (
	"log"
	"strconv"
	"strings"
)

type D6 struct {
}

func (d D6) PartOne(in []string) (int, int) {
	input := strings.Split(in[0], ",")

	var fish [10]int

	for _, val := range input {
		intval, err := strconv.Atoi(val)
		if err != nil {
			log.Fatal(err)
		}
		fish[intval] += 1
	}

	counter := 0
	counter2 := 0

	for j := 0; j < 80; j++ {
		tmp := fish[0]
		for ii := 0; ii < 8; ii++ {
			fish[ii] = fish[ii+1]
		}
		fish[6] += tmp
		fish[8] = tmp
	}

	for _, fi := range fish {
		counter += fi
	}

	var fish2 [9]int

	for _, val := range input {
		intval, err := strconv.Atoi(val)
		if err != nil {
			log.Fatal(err)
		}
		fish2[intval]++
	}

	for j := 0; j < 256; j++ {
		tmp := fish2[0]
		for ii := 0; ii < 8; ii++ {
			fish2[ii] = fish2[ii+1]
		}
		fish2[6] += tmp
		fish2[8] = tmp
	}

	for fi := range fish2 {
		counter2 += fish2[fi]
	}

	return counter, counter2
}
