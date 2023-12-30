package day12

import (
	"strconv"
	"strings"
)

type Record struct {
	possibleArrangementsByStartingIndex map[[2]int]int
	contiguousDamagedGroups             []int
	springs                             string
}

func (self Record) arrangements(index int, groupsFrom int) int {
	groups := self.contiguousDamagedGroups[groupsFrom:]
	if len(groups) < 1 {
		if index < len(self.springs) && strings.ContainsRune(self.springs[index:], '#') {
			return 0
		} else {
			return 1
		}
	}
	for index < len(self.springs) && self.springs[index] == '.' {
		index++
	}
	if index >= len(self.springs) {
		return 0
	}
	if memo, ok := self.possibleArrangementsByStartingIndex[[2]int{index, groupsFrom}]; ok {
		return memo
	}
	canFill := true
	for i := index; i < index+groups[0]; i++ {
		if i >= len(self.springs) || self.springs[i] == '.' {
			canFill = false
			break
		}
	}
	canFill = canFill && (index+groups[0] == len(self.springs) || self.springs[index+groups[0]] != '#')
	result := 0
	if canFill {
		result += self.arrangements(index+groups[0]+1, groupsFrom+1)
	}
	if self.springs[index] == '?' {
		result += self.arrangements(index+1, groupsFrom)
	}
	self.possibleArrangementsByStartingIndex[[2]int{index, groupsFrom}] = result
	return result
}

func (self Record) unfold() Record {
	contiguousDamaged := self.contiguousDamagedGroups
	springs := self.springs
	for i := 1; i < 5; i++ {
		contiguousDamaged = append(contiguousDamaged, self.contiguousDamagedGroups...)
		springs += "?" + self.springs
	}
	return Record{
		map[[2]int]int{},
		contiguousDamaged,
		springs,
	}
}

func RecordFromString(s string) Record {
	springs, damaged, _ := strings.Cut(s, " ")
	contiguousDamaged := []int{}
	for _, d := range strings.Split(damaged, ",") {
		contiguous, _ := strconv.Atoi(d)
		contiguousDamaged = append(contiguousDamaged, contiguous)
	}
	return Record{map[[2]int]int{}, contiguousDamaged, springs}
}

func Part01(input string) int {
	lines := strings.Split(input, "\n")
	arrangements := 0
	for _, line := range lines {
		arrangements += RecordFromString(line).arrangements(0, 0)
	}
	return arrangements
}
