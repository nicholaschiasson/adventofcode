package day02

import (
	"strconv"
	"strings"
)

func Part02(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		_, cubeGame, _ := strings.Cut(line, ": ")
		fewest := make(map[string]int)
		for _, cubeSet := range strings.Split(cubeGame, "; ") {
			for _, cubes := range strings.Split(cubeSet, ", ") {
				cubeCount, cubeColor, _ := strings.Cut(cubes, " ")
				nCubeCount, _ := strconv.Atoi(cubeCount)
				if nFewest, ok := fewest[cubeColor]; !ok || nCubeCount > nFewest {
					fewest[cubeColor] = nCubeCount
				}
			}
		}
		product := 1
		for _, cubeCount := range fewest {
			product *= cubeCount
		}
		sum += product
	}
	return sum
}
