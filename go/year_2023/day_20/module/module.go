package module

import (
	"fmt"
)

type Pulse = bool

const (
	High Pulse = true
	Low  Pulse = false
)

type State = bool

const (
	Off State = false
	On  State = true
)

type Type = int

const (
	Broadcast Type = iota
	Conjunction
	FlipFlop
)

const Broadcaster string = "broadcaster"

type Message struct {
	Pulse    Pulse
	Receiver Module
	Sender   Module
}

type Module interface {
	addSender(sender Module)
	AddReceiver(receiver Module)
	GetReceivers() []Module
	Name() string
	Send(message Message) []Message
}

type baseModule struct {
	name      string
	receivers []Module
}

func (self *baseModule) addSender(sender Module) {}

func (self *baseModule) AddReceiver(receiver Module) {
	self.receivers = append(self.receivers, receiver)
	if receiver != nil {
		receiver.addSender(self)
	}
}

func (self *baseModule) GetReceivers() []Module {
	return self.receivers
}

func (self *baseModule) Name() string {
	return self.name
}

func (self *baseModule) Send(message Message) []Message {
	messages := []Message{}
	for _, receiver := range self.receivers {
		messages = append(messages, Message{
			Pulse:    message.Pulse,
			Receiver: receiver,
			Sender:   self,
		})
	}
	return messages
}

type BroadcastModule struct {
	baseModule
}

type ConjunctionModule struct {
	baseModule
	previousPulses map[string]Pulse
}

func (self *ConjunctionModule) addSender(sender Module) {
	self.previousPulses[sender.Name()] = Low
}

func (self *ConjunctionModule) Send(message Message) []Message {
	self.previousPulses[message.Sender.Name()] = message.Pulse
	pulse := Low
	for _, p := range self.previousPulses {
		if p == Low {
			pulse = High
		}
	}
	messages := []Message{}
	for _, receiver := range self.receivers {
		messages = append(messages, Message{
			Pulse:    pulse,
			Receiver: receiver,
			Sender:   self,
		})
	}
	return messages
}

type FlipFlopModule struct {
	baseModule
	state State
}

func (self *FlipFlopModule) Send(message Message) []Message {
	messages := []Message{}
	if message.Pulse == High {
		return messages
	}
	self.state = !self.state
	pulse := Low
	if self.state == On {
		pulse = High
	}
	for _, receiver := range self.receivers {
		messages = append(messages, Message{
			Pulse:    pulse,
			Receiver: receiver,
			Sender:   self,
		})
	}
	return messages
}

func New(moduleType Type, name string) Module {
	switch moduleType {
	case Broadcast:
		return &BroadcastModule{
			baseModule: baseModule{
				name: name,
			},
		}
	case Conjunction:
		return &ConjunctionModule{
			baseModule: baseModule{
				name: name,
			},
			previousPulses: map[string]Pulse{},
		}
	case FlipFlop:
		return &FlipFlopModule{
			baseModule: baseModule{
				name: name,
			},
		}
	default:
		panic(fmt.Sprintf("invalid module type '%v'", moduleType))
	}
}
