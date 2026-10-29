package day01

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/nicholaschiasson/adventofcode/utils"
)

func Part02(input string) int {
	dial := 50
	zeroes := 0

	for _, line := range strings.Split(input, "\n") {
		i, err := strconv.Atoi(line[1:])
		utils.Check(err)
		sign := 1

		switch line[0] {
		case 'L':
			sign = -1
		case 'R':
		default:
			panic(fmt.Sprintf("Invalid dial turn: '%c'", line[0]))
		}

		prevDiff := dial
		if sign < 0 {
			prevDiff = (100 - dial) % 100
		}

		fmt.Println(dial, sign, i, prevDiff, (prevDiff+i)/100, ((dial+(sign*i))%100+100)%100)
		zeroes += (prevDiff + i) / 100

		dial = ((dial+(sign*i))%100 + 100) % 100
	}

	return zeroes
}
