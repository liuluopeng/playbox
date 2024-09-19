package main

import (
	"go_dsa/graph"
	Node "go_dsa/graph/Node"
)

func main() {
	g := graph.LoadFromArr([][]int{

		[]int{2, 4},
		[]int{1, 3},
		[]int{2, 4},
		[]int{1, 3},
	})

	// fmt.Println(*g)

	g.DepthFirst()





}


