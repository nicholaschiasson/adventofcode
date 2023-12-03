package day01

import (
	"fmt"
	"strconv"
	"strings"
	"unicode"

	"github.com/nicholaschiasson/adventofcode/utils"
)

func Part02(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		var translated strings.Builder
		for i := 0; i < len(line); i++ {
			l := line[i:]
			switch {
			case strings.HasPrefix(l, "one"):
				translated.WriteRune('1')
			case strings.HasPrefix(l, "two"):
				translated.WriteRune('2')
			case strings.HasPrefix(l, "three"):
				translated.WriteRune('3')
			case strings.HasPrefix(l, "four"):
				translated.WriteRune('4')
			case strings.HasPrefix(l, "five"):
				translated.WriteRune('5')
			case strings.HasPrefix(l, "six"):
				translated.WriteRune('6')
			case strings.HasPrefix(l, "seven"):
				translated.WriteRune('7')
			case strings.HasPrefix(l, "eight"):
				translated.WriteRune('8')
			case strings.HasPrefix(l, "nine"):
				translated.WriteRune('9')
			default:
				translated.WriteString(line[i : i+1])
			}
		}
		first, last := -1, -1
		l := []rune(translated.String())
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
