package day19

import (
	"strings"
)

type Range struct {
	low  int
	high int
}

func (self *Range) Size() int {
	if self.low > self.high {
		return 0
	}
	return self.high - self.low + 1
}

type RangeMap map[string]Range

func countCombinations(workflow string, ranges map[string]Range) int {
	if workflow == Rejected {
		return 0
	}
	if workflow == Accepted {
		x := ranges["x"]
		m := ranges["m"]
		a := ranges["a"]
		s := ranges["s"]
		return x.Size() * m.Size() * a.Size() * s.Size()
	}

	combinations := 0

	rangesCopy := map[string]Range{}
	for k, v := range ranges {
		rangesCopy[k] = v
	}
	w := workflows[workflow]
	for _, rule := range w.rules {
		rangesLoopCopy := map[string]Range{}
		for k, v := range rangesCopy {
			rangesLoopCopy[k] = v
		}
		if condition := rule.condition; condition != nil {
			switch condition.operator {
			case GreaterThan:
				if rng := rangesLoopCopy[condition.lval]; rng.low <= condition.rval {
					rangesLoopCopy[condition.lval] = Range{low: condition.rval + 1, high: rng.high}
					if rng := rangesLoopCopy[condition.lval]; rng.Size() > 0 {
						combinations += countCombinations(rule.outcome, rangesLoopCopy)
						rangesCopy[condition.lval] = Range{low: rangesCopy[condition.lval].low, high: condition.rval}
					}
				}
			case LessThan:
				if rng := rangesLoopCopy[condition.lval]; rng.high >= condition.rval {
					rangesLoopCopy[condition.lval] = Range{low: rng.low, high: condition.rval - 1}
					if rng := rangesLoopCopy[condition.lval]; rng.Size() > 0 {
						combinations += countCombinations(rule.outcome, rangesLoopCopy)
						rangesCopy[condition.lval] = Range{low: condition.rval, high: rangesCopy[condition.lval].high}
					}
				}
			}
		} else {
			combinations += countCombinations(rule.outcome, rangesLoopCopy)
		}
	}

	return combinations
}

func Part02(input string) int {
	workflowsInput, _, _ := strings.Cut(input, "\n\n")
	workflowsLines := strings.Split(workflowsInput, "\n")

	for _, workflowLine := range workflowsLines {
		name, workflow, _ := strings.Cut(workflowLine, "{")
		workflows[name] = ParseWorkflow(workflow)
	}

	return countCombinations("in", map[string]Range{
		"x": {1, 4000},
		"m": {1, 4000},
		"a": {1, 4000},
		"s": {1, 4000},
	})
}
