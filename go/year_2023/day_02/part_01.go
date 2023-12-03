package day02

import (
	"strconv"
	"strings"
)

func Part01(input string) int {
	lines := strings.Split(input, "\n")
	bag := map[string]int{"red": 12, "green": 13, "blue": 14}
	sum := 0
GAME:
	for i, line := range lines {
		_, cubeGame, _ := strings.Cut(line, ": ")
		for _, cubeSet := range strings.Split(cubeGame, "; ") {
			for _, cubes := range strings.Split(cubeSet, ", ") {
				cubeCount, cubeColor, _ := strings.Cut(cubes, " ")
				nCubeCount, _ := strconv.Atoi(cubeCount)
				if nCubeCount > bag[cubeColor] {
					continue GAME
				}
			}
		}
		sum += i + 1
	}
	return sum
}
