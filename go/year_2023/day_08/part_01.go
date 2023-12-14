package day09

import (
	"fmt"
	"strings"
)

type Instruction = int

const (
	L Instruction = iota
	R
)

func parseInput(input string) ([]Instruction, map[string][2]string) {
	instructions := []Instruction{}
	network := map[string][2]string{}
	if i, n, ok := strings.Cut(input, "\n\n"); ok {
		for _, i := range i {
			var instruction Instruction
			switch i {
			case 'L':
				instruction = L
			case 'R':
				instruction = R
			default:
				panic(fmt.Sprintf("invalid instruction '%c'", i))
			}
			instructions = append(instructions, instruction)
		}
		for _, node := range strings.Split(n, "\n") {
			key, next, _ := strings.Cut(node, " = ")
			next = next[1 : len(next)-1]
			left, right, _ := strings.Cut(next, ", ")
			network[key] = [2]string{left, right}
		}
	}
	return instructions, network
}

func Part01(input string) int {
	instructions, network := parseInput(input)
	steps := 0
	for node := "AAA"; node != "ZZZ"; steps++ {
		node = network[node][instructions[steps%len(instructions)]]
	}
	return steps
}
