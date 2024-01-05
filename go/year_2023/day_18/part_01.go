package day18

import (
	"fmt"
	"math"
	"strconv"
	"strings"

	. "github.com/nicholaschiasson/adventofcode/year_2023/day_18/vector"
)

type Direction = int

const (
	Right int = iota
	Down
	Left
	Up
)

type InstructionString string
type Instruction struct {
	direction Direction
	magnitude int
}

func (self InstructionString) Parse() Instruction {
	instruction := strings.Split(string(self), " ")
	magnitude, _ := strconv.Atoi(instruction[1])
	switch instruction[0] {
	case "U":
		return Instruction{Up, magnitude}
	case "L":
		return Instruction{Left, magnitude}
	case "R":
		return Instruction{Right, magnitude}
	case "D":
		return Instruction{Down, magnitude}
	default:
		panic(fmt.Sprintf("invalid direction '%v'", instruction[0]))
	}
}

func (self InstructionString) ParseHex() Instruction {
	instruction := strings.Split(string(self), " (#")[1]
	magnitude, _ := strconv.ParseInt(instruction[:5], 16, 0)
	return Instruction{Direction(instruction[5] - '0'), int(magnitude)}
}

func (self Instruction) Direction() Vector {
	switch self.direction {
	case Up:
		return UpVector()
	case Left:
		return LeftVector()
	case Right:
		return RightVector()
	case Down:
		return DownVector()
	default:
		panic(fmt.Sprintf("invalid direction '%v'", self.direction))
	}
}

func (self Instruction) Magnitude() int {
	return self.magnitude
}

func (self Instruction) Vector() Vector {
	return self.Direction().Scale(self.Magnitude())
}

func Part01(input string) int {
	lines := strings.Split(input, "\n")
	vertex := Vector([]int{0, 0})
	area2 := 0
	perimeter := 0
	for _, line := range lines {
		instruction := InstructionString(line).Parse()
		next := vertex.Add(instruction.Vector())
		area2 += vertex[0]*next[1] - vertex[1]*next[0]
		perimeter += instruction.Magnitude()
		vertex = next
	}
	return int(math.Abs(float64(area2)/2)) + (perimeter / 2) + 1
}
