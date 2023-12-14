package day09

import (
	"strings"
)

func gcd(a, b int) int {
	for b > 0 {
		a, b = b, a%b
	}
	return a
}

func lcm(a, b int) int {
	return a * b / gcd(a, b)
}

func Part02(input string) int {
	instructions, network := parseInput(input)
	ghosts := []string{}
	for k := range network {
		if strings.HasSuffix(k, "A") {
			ghosts = append(ghosts, k)
		}
	}
	steps := []int{}
	for i, ghost := range ghosts {
		for steps = append(steps, 0); !strings.HasSuffix(ghost, "Z"); steps[i]++ {
			ghost = network[ghost][instructions[steps[i]%len(instructions)]]
		}
	}
	stepsSet := map[int]bool{}
	for _, step := range steps {
		stepsSet[step] = true
	}
	leastSteps := 1
	for step := range stepsSet {
		leastSteps = lcm(leastSteps, step)
	}
	return leastSteps
}
