package day20

import (
	"strings"

	"github.com/nicholaschiasson/adventofcode/year_2023/day_20/module"
)

func pressButton(broadcaster module.Module, messageHook func(module.Message)) {
	messages := []module.Message{{Pulse: module.Low, Receiver: broadcaster}}
	for len(messages) > 0 {
		message := messages[0]
		messages = messages[1:]
		messageHook(message)
		if message.Receiver != nil {
			for _, m := range message.Receiver.Send(message) {
				messages = append(messages, m)
			}
		}
	}
}

func Part01(input string) int {
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
	lows, highs := 0, 0
	for i := 0; i < 1000; i++ {
		pressButton(modules[module.Broadcaster], func(message module.Message) {
			switch message.Pulse {
			case module.High:
				highs += 1
			case module.Low:
				lows += 1
			}
		})
	}
	return lows * highs
}
