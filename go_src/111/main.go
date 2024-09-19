package main

import (
	"fmt"
	"math"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func minDepth(node *TreeNode) int {

	if node == nil {
		return 0
	}
	if node.Left == nil && node.Right == nil {
		return 1
	}
	minDep := math.MaxInt32

	if node.Left != nil {
		minDep = min(minDep, minDepth(node.Left))
	}

	if node.Right != nil {
		minDep = min(minDep, minDepth(node.Right))
	}

	return minDep + 1
}

func min(x, y int) int {
	if x <= y {
		return x
	}
	return y
}

func minDepth3(node *TreeNode) int {

	if node == nil {
		return 0
	}
	if node.Left == nil && node.Right == nil {
		return 1
	}
	minDep := math.MaxInt32

	if node.Left != nil {
		if minDep > minDepth(node.Left) {
			minDep = minDepth(node.Left)
		} else {
			return minDep + 1
		}
	}

	if node.Right != nil {
		if minDep > minDepth(node.Right) {
			minDep = minDepth(node.Right)
		} else {
			return minDep + 1
		}
	}

	return minDep + 1
}

func minDepth2(root *TreeNode) int {

	type fp func(node *TreeNode) int
	var f fp
	f = func(node *TreeNode) int {

		if node == nil {
			return 0
		}
		if node.Left == nil && node.Right == nil {
			return 1
		}
		minDep := math.MaxInt32

		if node.Left != nil {
			if minDep > f(node.Left) {

				minDep = f(node.Left)
			}
		}

		if node.Right != nil {
			if minDep > f(node.Right) {
				minDep = f(node.Right)
			}
		}

		return minDep + 1
	}

	return f(root)
}

func main() {

	/**
				4
			3		2
		5		9		10
	90

	50
	*/
	root := &TreeNode{Val: 4}
	root.Left = &TreeNode{Val: 3}
	root.Left.Left = &TreeNode{Val: 5}
	root.Left.Left.Left = &TreeNode{Val: 90}
	root.Left.Left.Left.Left = &TreeNode{Val: 80}
	root.Right = &TreeNode{Val: 2}
	// root.Right.Left = &TreeNode{Val: 9}
	// root.Right.Right = &TreeNode{Val: 10}

	fmt.Println(minDepth(root))
}
