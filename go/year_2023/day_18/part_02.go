package day18

import (
	"math"
	"strings"

	. "github.com/nicholaschiasson/adventofcode/year_2023/day_18/vector"
)

func Part02(input string) int {
	lines := strings.Split(input, "\n")
	vertex := Vector([]int{0, 0})
	area2 := 0
	perimeter := 0
	for _, line := range lines {
		instruction := InstructionString(line).ParseHex()
		next := vertex.Add(instruction.Vector())
		area2 += vertex[0]*next[1] - vertex[1]*next[0]
		perimeter += instruction.Magnitude()
		vertex = next
	}
	return int(math.Abs(float64(area2)/2)) + (perimeter / 2) + 1
}
