package day20

import (
	"strings"

	"github.com/nicholaschiasson/adventofcode/year_2023/day_20/module"
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
	modules := map[string]module.Module{}
	for _, line := range strings.Split(input, "\n") {
		name, _, _ := strings.Cut(line, " -> ")
		switch {
		case name == module.Broadcaster:
			modules[name] = module.New(module.Broadcast, name)
		case name[0] == '%':
			modules[name[1:]] = module.New(module.FlipFlop, name[1:])
		case name[0] == '&':
			modules[name[1:]] = module.New(module.Conjunction, name[1:])
		}
	}
	for _, line := range strings.Split(input, "\n") {
		name, receivers, _ := strings.Cut(line, " -> ")
		name = strings.TrimLeftFunc(name, func(r rune) bool { return r == '%' || r == '&' })
		for _, receiver := range strings.Split(receivers, ", ") {
			modules[name].AddReceiver(modules[receiver])
		}
	}
	var outputModule module.Module
	for _, module := range modules {
		if module.GetReceivers()[0] == nil {
			outputModule = module
		}
	}
	senders := map[module.Module]bool{outputModule: true}
	fewestPresses := map[module.Module]int{}
	for presses := 1; len(senders) > 0; presses++ {
		delete(senders, outputModule)
		pressButton(modules[module.Broadcaster], func(m module.Message) {
			if m.Receiver == outputModule {
				if _, ok := fewestPresses[m.Sender]; !ok {
					senders[m.Sender] = true
				}
				if _, ok := senders[m.Sender]; ok && m.Pulse == module.High {
					fewestPresses[m.Sender] = presses
					delete(senders, m.Sender)
				}
			}
		})
	}
	fewestCommonPresses := 1
	for _, presses := range fewestPresses {
		fewestCommonPresses = lcm(fewestCommonPresses, presses)
	}
	return fewestCommonPresses
}
