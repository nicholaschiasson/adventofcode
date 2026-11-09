package day02

import (
	"math"
	"strconv"
	"strings"

	"github.com/nicholaschiasson/adventofcode/utils"
)

func Part01(input string) int {
	invalids := make([]int, 0)
	for _, r := range strings.Split(input, ",") {
		low, high, _ := strings.Cut(r, "-")
		lower, err := strconv.Atoi(low)
		utils.Check(err)
		upper, err := strconv.Atoi(high)
		utils.Check(err)
		for i := lower; i <= upper; i++ {
			digits := 0
			for j := i; j > 0; j /= 10 {
				digits++
			}
			half := float64(digits / 2)
			first := i / int(math.Pow(10, half))
			last := i - (first * int(math.Pow(10, half)))
			if first == last {
				invalids = append(invalids, i)
			}
		}
	}
	sum := 0
	for _, i := range invalids {
		sum += i
	}
	return sum
}
