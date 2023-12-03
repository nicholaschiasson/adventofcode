package day01

import (
	"fmt"
	"strconv"
	"strings"
	"unicode"

	"github.com/nicholaschiasson/adventofcode/utils"
)

func Part01(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		first, last := -1, -1
		l := []rune(line)
		for i := 0; (first < 0 || last < 0) && i < len(l); i++ {
			if first < 0 && unicode.IsDigit(l[i]) {
				first = i
			}
			if last < 0 && unicode.IsDigit(l[len(l)-i-1]) {
				last = len(l) - i - 1
			}
		}
		num, err := strconv.Atoi(fmt.Sprintf("%c%c", l[first], l[last]))
		utils.Check(err)
		sum += num
	}
	return sum
}
