package day17

import (
	"math"
	"strings"
)

type Node struct {
	weight     int
	neighbours [4]*Node
}

type Graph struct {
	nodes [][]*Node
}

type Direction = int

const (
	UP Direction = iota
	LEFT
	RIGHT
	DOWN
)

type Move struct {
	direction Direction
	step      int
	node      *Node
}

func leastHeatLoss(graph *Graph, sx, sy, dx, dy, minSteps, maxSteps int) int {
	start := Move{
		direction: -1,
		step:      0,
		node:      graph.nodes[sy][sx],
	}
	next := []Move{{
		direction: DOWN,
		step:      0,
		node:      graph.nodes[sy][sx],
	}, {
		direction: RIGHT,
		step:      0,
		node:      graph.nodes[sy][sx],
	}}
	visited := map[Move]int{start: 0}
	distance := math.MaxInt
	for len(next) > 0 {
		move := next[0]
		next = next[1:]
		if move.node == graph.nodes[dy][dx] && move.step >= minSteps && visited[move] < distance {
			distance = visited[move]
		}
		for i, neighbour := range move.node.neighbours {
			oppositeDir := ((((i * 2) - 3) * (-1)) + 3) / 2
			if neighbour == nil || oppositeDir == move.direction || (i == move.direction && move.step >= maxSteps) || (i != move.direction && move.step < minSteps) {
				continue
			}
			dist := visited[move] + neighbour.weight
			step := 1
			if i == move.direction {
				step = move.step + 1
			}
			m := Move{
				direction: i,
				step:      step,
				node:      neighbour,
			}
			if d, ok := visited[m]; !ok || dist < d {
				next = append(next, m)
				visited[m] = dist
			}
		}
	}
	return int(distance)
}

func Part01(input string) int {
	graph := Graph{
		nodes: [][]*Node{},
	}
	lines := strings.Split(input, "\n")
	for i, line := range lines {
		graph.nodes = append(graph.nodes, []*Node{})
		for j, char := range line {
			weight := int(char - '0')
			graph.nodes[i] = append(graph.nodes[i], &Node{
				weight:     weight,
				neighbours: [4]*Node{},
			})
			if i > 0 {
				graph.nodes[i-1][j].neighbours[DOWN] = graph.nodes[i][j]
				graph.nodes[i][j].neighbours[UP] = graph.nodes[i-1][j]
			}
			if j > 0 {
				graph.nodes[i][j-1].neighbours[RIGHT] = graph.nodes[i][j]
				graph.nodes[i][j].neighbours[LEFT] = graph.nodes[i][j-1]
			}
		}
	}
	return leastHeatLoss(&graph, 0, 0, len(graph.nodes[0])-1, len(graph.nodes)-1, 1, 3)
}
