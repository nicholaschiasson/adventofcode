package day09

import (
	"strconv"
	"strings"
)

func Part02(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		history := [][]int{{}}
		for _, num := range strings.Split(line, " ") {
			n, _ := strconv.Atoi(num)
			history[0] = append(history[0], n)
		}
		for allZero, i := false, 0; !allZero; i++ {
			allZero = true
			history = append(history, []int{})
			for j := range history[i][:len(history[i])-1] {
				diff := history[i][j+1] - history[i][j]
				history[i+1] = append(history[i+1], diff)
				if diff != 0 {
					allZero = false
				}
			}
		}
		for i := range history[:len(history)-1] {
			j := len(history) - 1 - i
			k := j - 1
			next := history[k][0] - history[j][len(history[j])-1]
			history[k] = append(history[k], next)
		}
		sum += history[0][len(history[0])-1]
	}
	return sum
}
