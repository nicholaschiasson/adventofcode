package day01

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/nicholaschiasson/adventofcode/utils"
)

func Part01(input string) int {
	dial := 50
	zeroes := 0

	for _, line := range strings.Split(input, "\n") {
		i, err := strconv.Atoi(line[1:])
		utils.Check(err)
		switch line[0] {
		case 'L':
			dial = (dial - i) % 100
		case 'R':
			dial = (dial + i) % 100
		default:
			panic(fmt.Sprintf("Invalid dial turn: '%c'", line[0]))
		}
		if dial == 0 {
			zeroes++
		}
	}

	return zeroes
}
