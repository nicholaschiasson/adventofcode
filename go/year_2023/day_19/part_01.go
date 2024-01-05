package day19

import (
	"fmt"
	"strconv"
	"strings"
)

type Part struct {
	x int
	m int
	a int
	s int
}

func (self *Part) Get(field string) int {
	switch field {
	case "x":
		return self.x
	case "m":
		return self.m
	case "a":
		return self.a
	case "s":
		return self.s
	default:
		panic(fmt.Sprintf("attempt to get invalid field '%v'", field))
	}
}

func (self *Part) Set(field string, value int) {
	switch field {
	case "x":
		self.x = value
	case "m":
		self.m = value
	case "a":
		self.a = value
	case "s":
		self.s = value
	default:
		panic(fmt.Sprintf("attempt to set invalid field '%v'", field))
	}
}

func ParsePart(str string) Part {
	fields := strings.Split(strings.Trim(str, "{}"), ",")
	_, fx, _ := strings.Cut(fields[0], "=")
	_, fm, _ := strings.Cut(fields[1], "=")
	_, fa, _ := strings.Cut(fields[2], "=")
	_, fs, _ := strings.Cut(fields[3], "=")
	x, _ := strconv.Atoi(fx)
	m, _ := strconv.Atoi(fm)
	a, _ := strconv.Atoi(fa)
	s, _ := strconv.Atoi(fs)
	return Part{x, m, a, s}
}

type Operator = byte

const (
	GreaterThan Operator = iota
	LessThan
)

type Condition struct {
	lval     string
	operator Operator
	rval     int
}

func (self *Condition) Evaluate(part Part) bool {
	lval := part.Get(self.lval)
	switch self.operator {
	case GreaterThan:
		return lval > self.rval
	case LessThan:
		return lval < self.rval
	default:
		panic(fmt.Sprintf("invalid operator '%v'", self.operator))
	}
}

func ParseCondition(s string) *Condition {
	if l, r, ok := strings.Cut(s, "<"); ok {
		right, _ := strconv.Atoi(r)
		return &Condition{l, LessThan, right}
	} else if l, r, ok := strings.Cut(s, ">"); ok {
		right, _ := strconv.Atoi(r)
		return &Condition{l, GreaterThan, right}
	} else {
		panic(fmt.Sprintf("invalid condition '%v'", s))
	}
}

type Outcome = string

const (
	Accepted Outcome = "A"
	Rejected Outcome = "R"
)

type Rule struct {
	condition *Condition
	outcome   Outcome
}

func (self *Rule) Evaluate(part Part) bool {
	if self.condition != nil {
		return self.condition.Evaluate(part)
	}
	return true
}

func (self *Rule) Resolve(part Part) Outcome {
	return self.outcome
}

func ParseRule(s string) Rule {
	cond, outcome, ok := strings.Cut(s, ":")
	var condition *Condition
	if ok {
		condition = ParseCondition(cond)
	} else {
		outcome = cond
	}
	return Rule{condition, outcome}
}

type Workflow struct {
	rules []Rule
}

func (self *Workflow) Run(part Part) Outcome {
	for _, rule := range self.rules {
		if rule.Evaluate(part) {
			outcome := rule.Resolve(part)
			for outcome != Accepted && outcome != Rejected {
				workflow := workflows[outcome]
				outcome = workflow.Run(part)
			}
			return outcome
		}
	}
	panic("no rules satisfied")
}

func ParseWorkflow(s string) Workflow {
	ruleStrings := strings.Split(strings.Trim(s, "{}"), ",")
	rules := []Rule{}
	for _, rule := range ruleStrings {
		rules = append(rules, ParseRule(rule))
	}
	return Workflow{rules}
}

var workflows map[string]Workflow = map[string]Workflow{}

func Part01(input string) int {
	workflowsInput, partsInput, _ := strings.Cut(input, "\n\n")
	workflowsLines := strings.Split(workflowsInput, "\n")
	partsLines := strings.Split(partsInput, "\n")

	for _, workflowLine := range workflowsLines {
		name, workflow, _ := strings.Cut(workflowLine, "{")
		workflows[name] = ParseWorkflow(workflow)
	}

	in := workflows["in"]
	sum := 0
	for _, partLine := range partsLines {
		part := ParsePart(partLine)
		if in.Run(part) == Accepted {
			sum += part.x + part.m + part.a + part.s
		}
	}

	return sum
}
