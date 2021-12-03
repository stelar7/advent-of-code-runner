package d3

import (
	"log"
	"strconv"
)

type D3 struct {
}

func (d D3) PartOne(in []string) int64 {

	gamma := ""
	epsilon := ""

	for i := 0; i < len(in[0]); i++ {
		zeroes := 0
		ones := 0

		for line := range in {
			text := in[line]
			if string([]rune(text)[i]) == "0" {
				zeroes += 1
			}
			if string([]rune(text)[i]) == "1" {
				ones += 1
			}
		}
		if zeroes > ones {
			gamma += "0"
			epsilon += "1"
		} else {
			gamma += "1"
			epsilon += "0"
		}
	}
	gamma_decimal, err := strconv.ParseInt(gamma, 2, 64)
	if err != nil {
		log.Fatal(err)
	}
	epsilon_decimal, err := strconv.ParseInt(epsilon, 2, 64)
	if err != nil {
		log.Fatal(err)
	}
	println(epsilon_decimal, gamma_decimal)
	return gamma_decimal * epsilon_decimal
}

func (d D3) PartTwo(num int, in []string, oxco string) string {
	ones := 0
	zeroes := 0
	var onenum []string
	var zeronum []string
	println(len(in), "-")

	if len(in) == 1 {
		return in[0]
	}

	for i := range in {
		text := in[i]
		letter := text[num : num+1]
		if letter == "1" {
			ones += 1
			onenum = append(onenum, text)
		}
		if letter == "0" {
			zeroes += 1
			zeronum = append(zeronum, text)
		}
	}

	if ones > zeroes {
		if oxco == "ox" {
			return d.PartTwo(num+1, onenum, "ox")
		} else {
			return d.PartTwo(num+1, zeronum, "co")
		}
	} else if zeroes > ones {
		if oxco == "ox" {
			return d.PartTwo(num+1, zeronum, "ox")
		} else {
			return d.PartTwo(num+1, onenum, "co")
		}
	} else if zeroes == ones {
		if oxco == "ox" {
			return d.PartTwo(num+1, onenum, "ox")
		} else {
			return d.PartTwo(num+1, zeronum, "co")
		}
	}

	if oxco == "ox" {
		return onenum[0]
	} else {
		return zeronum[0]
	}
}
