package day03

import (
	"math"
	"strconv"
	"strings"
	"unicode"
)

func Part02(input string) int {
	lines := strings.Split(input, "\n")
	schematic := make([][]int, len(lines))
	for i, line := range lines {
		schematic[i] = make([]int, len(line))
		for j, c := range line {
			if schematic[i][j] != 0 {
				continue
			}
			if unicode.IsDigit(c) {
				width := j
				for _, c := range line[j:] {
					if !unicode.IsDigit(c) {
						break
					}
					width++
				}
				numStr := line[j:width]
				num, _ := strconv.Atoi(numStr)
				for k := range numStr {
					schematic[i][j+k] = num
				}
			}
		}
	}
	sum := 0
	for i, line := range lines {
		for j, c := range line {
			if c != '.' && !unicode.IsDigit(c) {
				partNumbers := []int{}
				for y := math.Max(0, float64(i)-1); y < math.Min(float64(len(lines)), float64(i)+2); y++ {
					overlapping := false
					for x := math.Max(0, float64(j)-1); x < math.Min(float64(len(line)), float64(j)+2); x++ {
						num := schematic[int(y)][int(x)]
						if !overlapping && num != 0 {
							partNumbers = append(partNumbers, num)
						}
						overlapping = num != 0
					}
				}
				if c == '*' && len(partNumbers) == 2 {
					sum += partNumbers[0] * partNumbers[1]
				}
			}
		}
	}
	return sum
}
