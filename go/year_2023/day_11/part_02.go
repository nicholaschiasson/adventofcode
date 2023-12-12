package day11

import (
	"math"
	"strings"
)

func Part02(input string) int {
	lines := strings.Split(input, "\n")
	rowPositions := []int{}
	columnPositions := []int{}
	galaxies := []Galaxy{}
	for y, line := range lines {
		rowPositions = append(rowPositions, 1000000)
		for x, char := range line {
			if x >= len(columnPositions) {
				columnPositions = append(columnPositions, 1000000)
			}
			if char == '#' {
				galaxies = append(galaxies, Galaxy{
					Position{
						x, y,
					},
				})
				columnPositions[x] = 1
				rowPositions[y] = 1
			}
		}
	}
	for i := 0; i < len(rowPositions)-1; i++ {
		rowPositions[i+1] += rowPositions[i]
	}
	for i := 0; i < len(columnPositions)-1; i++ {
		columnPositions[i+1] += columnPositions[i]
	}
	for i := 0; i < len(galaxies); i++ {
		galaxies[i] = Galaxy{
			Position{
				x: columnPositions[galaxies[i].position.x] - 1,
				y: rowPositions[galaxies[i].position.y] - 1,
			},
		}
	}
	sum := 0
	for i := 0; i < len(galaxies)-1; i++ {
		for j := i + 1; j < len(galaxies); j++ {
			xI, yI := float64(galaxies[i].position.x), float64(galaxies[i].position.y)
			xJ, yJ := float64(galaxies[j].position.x), float64(galaxies[j].position.y)
			sum += int(math.Abs(xJ-xI) + math.Abs(yJ-yI))
		}
	}
	return sum
}
