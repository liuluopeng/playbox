package main

func mian() {

}

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

type Node struct {
	Val      int
	Children []*Node
}

func levelOrder(root *Node) [][]int {
	res := make([][]int, 0)
	queue := make([]*Node, 0)

	if root != nil {
		queue = append(queue, root)
	}

	for len(queue) != 0 {

		tmpQueue := make([]*Node, 0)

		for _, node := range queue {

			for _, child := range node.Children {
				if child != nil {
					tmpQueue = append(tmpQueue, child)
				}
			}
		}

		level := make([]int, 0)
		for _, node := range queue {
			level = append(level, node.Val)
		}
		res = append(res, level)
		queue = tmpQueue
	}

	return res
}
