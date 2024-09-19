package main

import (
	"fmt"
)

/**
 * Definition for a QuadTree node.
 * type Node struct {
 *     Val bool
 *     IsLeaf bool
 *     TopLeft *Node
 *     TopRight *Node
 *     BottomLeft *Node
 *     BottomRight *Node
 * }
 */

func main() {
	grid := [][]int{
		{1, 1, 1, 1, 0, 0, 0, 0},
		{1, 1, 1, 1, 0, 0, 0, 0},
		{1, 1, 1, 1, 1, 1, 1, 1},
		{1, 1, 1, 1, 1, 1, 1, 1},
		{1, 1, 1, 1, 0, 0, 0, 0},
		{1, 1, 1, 1, 0, 0, 0, 0},
		{1, 1, 1, 1, 0, 0, 0, 0},
		{1, 1, 1, 1, 0, 0, 0, 0},
	}

	grid = [][]int{
		{1, 1, 0, 0},
		{0, 0, 1, 1},
		{1, 1, 0, 0},
		{0, 0, 1, 1},
	}

	rootNode := construct(grid)
	fmt.Println(rootNode)
	rootNode.bfs()
}

type Node struct {
	IsLeaf bool
	Val    bool

	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

func construct(grid [][]int) *Node {
	// grid是一个2的n次方倍数的正方形

	if len(grid) == 0 {
		return nil
	}

	if len(grid) == 1 {
		val := false
		if grid[0][0] == 1 {
			val = true
		}
		// 构造一个叶子

		ret := &Node{
			IsLeaf: true,
			Val:    val,
		}

		fmt.Println("零碎", ret)

		return ret
	}

	cutList := cut(grid)

	// for _, c := range cutList {
	// 	pp_sq(c)
	// }

	r := make([]*Node, 0)

	for _, s := range cutList {
		// pp_sq(s)
		r = append(r, construct(s))
	}

	rootNode := &Node{}
	// 看看四个结果一不一样
	// 一样的话 isLeaf 是 true
	if r[0].IsLeaf && r[1].IsLeaf && r[2].IsLeaf && r[3].IsLeaf && r[0].Val == r[1].Val && r[1].Val == r[2].Val && r[2].Val == r[3].Val {
		rootNode.IsLeaf = true

		rootNode.Val = r[0].Val

		rootNode.TopLeft = nil
		rootNode.TopRight = nil
		rootNode.BottomLeft = nil
		rootNode.BottomRight = nil
	} else { // 四个方块不同的话,'
		rootNode.IsLeaf = false
		rootNode.Val =  true // 随便

		rootNode.TopLeft = r[0]
		rootNode.TopRight = r[1]
		rootNode.BottomLeft = r[2]
		rootNode.BottomRight = r[3]

		// fmt.Println("构造父亲结点", "左上方的孩子列表是", cutList[0])
	}

	// fmt.Println(rootNode)

	fmt.Print(rootNode.Val)
	rootNode.bfs()

	return rootNode
}

// 把一个方块分割成四块
func cut(grid [][]int) [][][]int {
	n := len(grid)
	half := n / 2

	// Initialize the four quadrants as slices of the original grid
	topLeft := make([][]int, half)
	topRight := make([][]int, half)
	bottomLeft := make([][]int, half)
	bottomRight := make([][]int, half)

	for i := 0; i < half; i++ {
		topLeft[i] = grid[i][:half]
		topRight[i] = grid[i][half:]
		bottomLeft[i] = grid[i+half][:half]
		bottomRight[i] = grid[i+half][half:]
	}

	res := [][][]int{topLeft, topRight, bottomLeft, bottomRight}
	return res
}

func pp_sq(grid [][]int) {
	for i, c := range grid {
		fmt.Println(i, c)
	}
	// fmt.Println()
}

func (node *Node) bfs() {
	queue := make([]*Node, 0)
	res := make([][]int, 0)
	if node != nil {
		queue = append(queue, node)
	}

	for len(queue) != 0 {
		currentNode := queue[0]
		queue = queue[1:]

		one := make([]int, 0)
		if currentNode.IsLeaf {
			one = append(one, 1)
		} else {
			one = append(one, 0)
		}
		if currentNode.Val {
			one = append(one, 1)
		} else {
			one = append(one, 0)
		}
		res = append(res, one)

		if currentNode.BottomLeft != nil {
			queue = append(queue, currentNode.BottomLeft)
		}
		if currentNode.BottomRight != nil {
			queue = append(queue, currentNode.BottomRight)
		}
		if currentNode.TopLeft != nil {
			queue = append(queue, currentNode.TopLeft)

		}
		if currentNode.TopRight != nil {
			queue = append(queue, currentNode.TopRight)
		}
	}

	fmt.Println(res)
}
