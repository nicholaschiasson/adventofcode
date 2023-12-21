package day17

import "strings"

func Part02(input string) int {
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
				graph.nodes[i-1][j].neighbours[Down] = graph.nodes[i][j]
				graph.nodes[i][j].neighbours[Up] = graph.nodes[i-1][j]
			}
			if j > 0 {
				graph.nodes[i][j-1].neighbours[Right] = graph.nodes[i][j]
				graph.nodes[i][j].neighbours[Left] = graph.nodes[i][j-1]
			}
		}
	}
	return leastHeatLoss(&graph, 0, 0, len(graph.nodes[0])-1, len(graph.nodes)-1, 4, 10)
}
