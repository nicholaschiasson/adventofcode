package day12

import (
	"strings"
)

func Part02(input string) int {
	lines := strings.Split(input, "\n")
	arrangements := 0
	for _, line := range lines {
		arrangements += RecordFromString(line).unfold().arrangements(0, 0)
	}
	return arrangements
}
